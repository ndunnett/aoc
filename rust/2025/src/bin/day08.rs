use std::mem::MaybeUninit;

type Coordinate = i32;
type Index = u32;

const GRID_SPACE: usize = 100000;
const GRID_DIVISIONS: usize = 8;
const GRID_WIDTH: usize = GRID_SPACE / GRID_DIVISIONS;
const CELLS_PER_AXIS: usize = GRID_DIVISIONS + 1;
const POINTS_N: usize = 1000;

#[derive(Clone)]
struct Edge {
    distance: Coordinate,
    i: Index,
    j: Index,
}

type LocalEdgesVec = MicroVec<Edge, 64, u8>;
type EdgesVec = MicroVec<Edge, 45000, usize>;
type CoordinateVec = MicroVec<Coordinate, POINTS_N, usize>;
type CellVec = MicroVec<Index, 20, u8>;
type DsuVec = MicroVec<Index, POINTS_N, usize>;

struct DisjointSetUnion {
    reps: DsuVec,
    sizes: DsuVec,
}

impl DisjointSetUnion {
    #[inline(always)]
    fn new(len: usize) -> Self {
        let reps = (0..len as Index).collect::<DsuVec>();
        // Safety: the entire backing array is initialised
        let sizes = unsafe { DsuVec::from_raw_parts([MaybeUninit::new(1); _], len) };
        assert!(reps.len() == sizes.len());
        Self { reps, sizes }
    }

    #[inline(always)]
    fn find(&mut self, i: Index) -> Index {
        let mut i = i as usize;

        // Follow joins in the set until you reach the root
        loop {
            // Safety: `i` and `parent` will never overrun `self.parent`
            unsafe {
                let parent = *self.reps.get_unchecked(i) as usize;

                if parent == i {
                    return i as Index;
                }

                let grandparent = *self.reps.get_unchecked(parent) as usize;
                *self.reps.get_unchecked_mut(i) = grandparent as Index;
                i = parent;
            }
        }
    }

    #[inline(always)]
    fn unite(&mut self, i: Index, j: Index) -> bool {
        // Find representatives of `i` and `j`
        let mut i_rep = self.find(i) as usize;
        let mut j_rep = self.find(j) as usize;

        // Early exit if they're already united
        if i_rep == j_rep {
            return false;
        }

        // Safety: `i_rep` and `j_rep` will never overrun `self.size` or `self.parent`
        unsafe {
            // Merge the smaller representative into the bigger representative
            let i_size = *self.sizes.get_unchecked(i_rep);
            let j_size = *self.sizes.get_unchecked(j_rep);

            if i_size < j_size {
                std::mem::swap(&mut i_rep, &mut j_rep);
            }

            *self.reps.get_unchecked_mut(j_rep) = i_rep as Index;
            *self.sizes.get_unchecked_mut(i_rep) += *self.sizes.get_unchecked(j_rep);
        }

        true
    }

    #[inline(always)]
    fn into_sizes(self) -> DsuVec {
        self.sizes
    }
}

#[derive(Clone)]
struct Solution {
    xs: CoordinateVec,
    edges: Box<EdgesVec>,
}

impl Solution {
    // Generic parameter for testing against sample input
    fn part1_solver<const N: usize>(&self) -> u32 {
        // Start with a DSU with an element for each point
        let n = self.xs.len();
        let mut dsu = DisjointSetUnion::new(n);

        // Unite the first `N` edges
        for &Edge { distance: _, i, j } in &self.edges[..N] {
            _ = dsu.unite(i, j);
        }

        // Gather size of disjoint sets
        let mut sizes = dsu.into_sizes();
        sizes.sort_unstable_by(|a, b| b.cmp(a));
        sizes[0..3].iter().product()
    }
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        // Parse points into vecs of coordinates
        let mut xs = CoordinateVec::new();
        let mut ys = CoordinateVec::new();
        let mut zs = CoordinateVec::new();

        for (x, y, z) in NumberParser::from(input).tuples() {
            unsafe {
                xs.push_unchecked(x);
                ys.push_unchecked(y);
                zs.push_unchecked(z);
            }
        }

        let n = xs.len();
        assert!(n == ys.len());
        assert!(n == zs.len());

        // Partition point indices into cells
        let mut cells =
            Box::new([[[CellVec::new(); CELLS_PER_AXIS]; CELLS_PER_AXIS]; CELLS_PER_AXIS]);

        for i in 0..n {
            let cx = xs[i] as usize / GRID_WIDTH;
            let cy = ys[i] as usize / GRID_WIDTH;
            let cz = zs[i] as usize / GRID_WIDTH;

            // Safety: points are sparse enough that the capacity of each `CellVec` should never be reached
            unsafe { cells[cx][cy][cz].push_unchecked(i as Index) };
        }

        // Build chunks of edges in parallel, only comparing each point to other points in the same or neighboring cells
        let edge_chunks: Vec<LocalEdgesVec> = (0..n)
            .into_par_iter()
            .map(|i| {
                let mut edges = LocalEdgesVec::new();

                // Determine which cell the point is in
                let cx = xs[i] as usize / GRID_WIDTH;
                let cy = ys[i] as usize / GRID_WIDTH;
                let cz = zs[i] as usize / GRID_WIDTH;

                // Iterate cell and neighbours
                let cx_start = cx.saturating_sub(1);
                let cx_end = (cx + 2).min(CELLS_PER_AXIS);

                for row in &cells[cx_start..cx_end] {
                    let cy_start = cy.saturating_sub(1);
                    let cy_end = (cy + 2).min(CELLS_PER_AXIS);

                    for col in &row[cy_start..cy_end] {
                        let cz_start = cz.saturating_sub(1);
                        let cz_end = (cz + 2).min(CELLS_PER_AXIS);

                        for cell in &col[cz_start..cz_end] {
                            // Iterate other points in the cell
                            for &j in cell {
                                let j = j as usize;

                                if j <= i {
                                    continue;
                                }

                                // Calculate distance between points and insert into edges
                                assert!(j < n);
                                let dx = xs[i] - xs[j];
                                let dy = ys[i] - ys[j];
                                let dz = zs[i] - zs[j];
                                let distance = dx * dx + dy * dy + dz * dz;
                                let i = i as Index;
                                let j = j as Index;
                                unsafe { edges.push_unchecked(Edge { distance, i, j }) };
                            }
                        }
                    }
                }

                edges
            })
            .collect();

        // Merge chunks of edges into a single vec
        let mut edges = Box::new(EdgesVec::new());

        for chunk in edge_chunks {
            for e in chunk.into_iter() {
                unsafe { edges.push_unchecked(e) };
            }
        }

        // Sort edges by distance
        edges.par_sort_unstable_by_key(|e| e.distance);
        Ok(Self { xs, edges })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        Ok(self.part1_solver::<1000>())
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        // Start with a DSU with an element for each point
        let n = self.xs.len();
        let mut dsu = DisjointSetUnion::new(n);
        let mut components = n;

        // Unite the DSU edge by edge until all elements are joined
        for &Edge { distance: _, i, j } in &self.edges {
            if dsu.unite(i, j) {
                components -= 1;

                if components == 1 {
                    return Ok(self.xs[i as usize] * self.xs[j as usize]);
                }
            }
        }

        unreachable!()
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn test_part1() {
        assert_eq!(Solution::new(INPUT).unwrap().part1_solver::<10>(), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Solution::new(INPUT).unwrap().part2().unwrap().to_string(),
            "25272"
        );
    }
}
