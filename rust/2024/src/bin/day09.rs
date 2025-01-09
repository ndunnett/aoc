#[derive(Copy, Clone)]
struct File {
    id: u16,
    size: u8,
}

impl File {
    fn new(id: u16, size: u8) -> Self {
        Self { id, size }
    }
}

#[derive(Clone)]
struct Page {
    files: [File; 9],
    start: u8,
    len: u8,
    free: u8,
}

impl Page {
    const NEW_FILES: [File; 9] = [File { id: 0, size: 0 }; 9];

    fn new(file: File, free: u8) -> Self {
        let mut data = Self::NEW_FILES;
        data[0] = file;

        Self {
            files: data,
            start: 0,
            len: 1,
            free,
        }
    }
}

struct Solution {
    fs: Vec<Page>,
}

impl Solution {
    // look up table ADD_SERIES[n] == 0 + 1 + 2 ... + n, where n < 10
    const ADD_SERIES: [usize; 10] = [
        0,
        1,
        1 + 2,
        1 + 2 + 3,
        1 + 2 + 3 + 4,
        1 + 2 + 3 + 4 + 5,
        1 + 2 + 3 + 4 + 5 + 6,
        1 + 2 + 3 + 4 + 5 + 6 + 7,
        1 + 2 + 3 + 4 + 5 + 6 + 7 + 8,
        1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9,
    ];
}

impl Solver for Solution {
    fn new(input: &str) -> Anyhow<Self> {
        Ok(Self {
            fs: (0..u16::MAX)
                .zip(input.trim_end().bytes().chain([b'0']).tuples())
                .map(|(id, (size, free))| Page::new(File::new(id, size - b'0'), free - b'0'))
                .collect(),
        })
    }

    fn part1(&mut self) -> Anyhow<impl fmt::Display> {
        let mut fs = self.fs.clone();
        let mut front = 0;
        let mut back = fs.len() - 1;

        // iterate from both ends of the fs vec, taking files from the back and inserting into free space at the front
        // when front and back indices meet in the middle, all free space is filled
        while front < back {
            // increment the front page index when the front page is full
            if fs[front].free == 0 {
                front += 1;
                continue;
            }

            // move the last file if there is one in the back page, otherwise decrement the back page index
            if fs[back].len > 0 {
                let i = fs[back].len as usize - 1;
                let j = fs[front].len as usize;

                // move the entire file if it fits, otherwise fragment the file
                if fs[back].files[i].size <= fs[front].free {
                    let id = fs[back].files[i].id;
                    let size = fs[back].files[i].size;

                    fs[front].files[j].id = id;
                    fs[front].files[j].size = size;
                    fs[front].len += 1;
                    fs[front].free -= size;

                    fs[back].len -= 1;
                    fs[back].free += size;
                } else {
                    let id = fs[back].files[i].id;
                    let size = fs[front].free;

                    fs[front].files[j].id = id;
                    fs[front].files[j].size = size;
                    fs[front].len += 1;
                    fs[front].free -= size;

                    fs[back].free += size;
                    fs[back].files[i].size -= size;
                }
            } else {
                back -= 1;
            }
        }

        let mut checksum = 0;
        let mut i = 0;

        // all pages after the last back page index are zero
        for page in &fs[0..=back] {
            // iterate over page from the start index
            // there is no free space so `i` only increases with file sizes
            for j in page.start as usize..page.len as usize {
                // file checksum, where i == first index of the file, j == 0..size:
                //   => (i + 0) * id + (i + 1) * id + (i + 2) * id ... + (i + j) * id
                //   => (i * size + 0 + 1 + 2 ... (size - 1)) * id
                let size = page.files[j].size as usize;
                checksum += (i * size + Self::ADD_SERIES[size - 1]) * page.files[j].id as usize;
                i += size;
            }
        }

        Ok(checksum)
    }

    fn part2(&mut self) -> Anyhow<impl fmt::Display> {
        let mut fs = self.fs.clone();
        let mut highest_unmoved = None;
        let mut lowest_free = [0_usize; 10];

        // reverse iterate over all pages
        for back in (0..fs.len()).rev() {
            let size = fs[back].files[0].size;

            // iterate from the lowest page index that may fit the first file from the current page, up to the current page
            for front in lowest_free[size as usize]..back {
                // move the first file from the current page to the front page if it fits
                if fs[front].free >= size {
                    let j = fs[front].len as usize;
                    let id = fs[back].files[0].id;

                    fs[front].files[j].id = id;
                    fs[front].files[j].size = size;
                    fs[front].len += 1;
                    fs[front].free -= size;

                    fs[back].start += 1;
                    fs[back].files[0].id = 0;

                    // track the lowest page index that can fit the given size
                    for i in lowest_free.iter_mut().skip(size as usize) {
                        if front > *i {
                            *i = front;
                        }
                    }

                    break;
                }
            }

            // track the highest page index which is not moved
            if highest_unmoved.is_none() && fs[back].start > 0 {
                highest_unmoved = Some(back);
            }
        }

        let mut checksum = 0;
        let mut i = 0;

        // all pages after the highest unmoved page are zero
        for page in &fs[0..=highest_unmoved.unwrap_or(fs.len() - 1)] {
            // iterate over whole page including empty pages at the start
            // `i` increments with file size and free space
            for j in 0..page.len as usize {
                // file checksum, where i == first index of the file, j == 0..size:
                //   => (i + 0) * id + (i + 1) * id + (i + 2) * id ... + (i + j) * id
                //   => (i * size + 0 + 1 + 2 ... (size - 1)) * id
                let size = page.files[j].size as usize;
                checksum += (i * size + Self::ADD_SERIES[size - 1]) * page.files[j].id as usize;
                i += size;
            }

            i += page.free as usize;
        }

        Ok(checksum)
    }
}

aoc::solution!();

#[cfg(test)]
mod test {
    use super::{Solution, Solver};

    const INPUT: &str = r"2333133121414131402";

    #[test]
    fn test_part1() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part1().unwrap().to_string();
        assert_eq!(answer, "1928");
    }

    #[test]
    fn test_part2() {
        let mut solution = Solution::new(INPUT).unwrap();
        let answer = solution.part2().unwrap().to_string();
        assert_eq!(answer, "2858");
    }
}
