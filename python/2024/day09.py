from dataclasses import dataclass
from itertools import batched


@dataclass
class File:
    id: int
    size: int


@dataclass
class Page:
    files: list[File]
    free: int

    def __len__(self) -> int:
        return len(self.files)


def parse(input: str) -> list[Page]:
    return [
        Page([File(_id, int(size))], int(free))
        for _id, (size, free) in enumerate(batched(input.strip() + "0", 2))
    ]


# Look up table ADD_SERIES[n] == 0 + 1 + 2 ... + n, where n < 10
ADD_SERIES: tuple[int, ...] = (
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
)


def part1(input: str) -> int:
    fs = parse(input)
    front = 0
    back = len(fs) - 1

    # Iterate from both ends of the fs vec, taking files from the back and inserting into
    # free space at the front. When the front and back indices meet in the middle, all free
    # space is filled.
    while front < back:
        # Increment the front page index when the front page is full.
        if fs[front].free == 0:
            front += 1
            continue

        # Move the last file if there is one in the back page, otherwise decrement the back
        # page index.
        if len(fs[back]) > 0:
            i = len(fs[back]) - 1

            # Move the entire file if it fits, otherwise fragment the file.
            if fs[back].files[i].size <= fs[front].free:
                _id = fs[back].files[i].id
                size = fs[back].files[i].size

                fs[front].files.append(File(_id, size))
                fs[front].free -= size

                fs[back].files.pop()
                fs[back].free += size
            else:
                _id = fs[back].files[i].id
                size = fs[front].free

                fs[front].files.append(File(_id, size))
                fs[front].free -= size

                fs[back].free += size
                fs[back].files[i].size -= size
        else:
            back -= 1

    checksum = 0
    i = 0

    # All pages after the last back page index are zero.
    for page in fs[0 : back + 1]:
        # Iterate over files in the page.
        # There is no free space so `i` only increases with file sizes.
        for j in range(len(page)):
            # File checksum, where i == first index of the file:
            #   => (i + 0) * id + (i + 1) * id + (i + 2) * id ... + (i + size) * id
            #   => (i * size + 0 + 1 + 2 ... (size - 1)) * id
            size = page.files[j].size
            checksum += (i * size + ADD_SERIES[size - 1]) * page.files[j].id
            i += size

    return checksum


def part2(input: str) -> int:
    fs = parse(input)
    highest_populated = 0
    lowest_free = [0] * 10

    # Reverse iterate over all pages.
    for back in reversed(range(len(fs))):
        size = fs[back].files[0].size

        # Iterate from the lowest page index that may fit the first file from the current
        # page, up to the current page index.
        for front in range(lowest_free[size], back):
            # Move the first file from the current page to the front page if it fits.
            if fs[front].free >= size:
                _id = fs[back].files[0].id
                fs[front].files.append(File(_id, size))
                fs[front].free -= size
                fs[back].files[0].id = 0

                # Track the lowest page index that can fit the given size.
                for i in range(size, len(lowest_free)):
                    lowest_free[i] = max(lowest_free[i], front)

                # Track the highest page index - moved front page.
                highest_populated = max(highest_populated, front)

                break

        # Track the highest page index - unmoved back page.
        if len(fs[back].files) > 0 and highest_populated < back:
            highest_populated = back

    checksum = 0
    i = 0

    # All pages after the highest populated page are zero.
    for page in fs[0 : highest_populated + 1]:
        # Iterate over the whole page including empty files at the start.
        # `i` increments with file size and free space.
        for j in range(len(page)):
            # File checksum, where i == first index of the file:
            #   => (i + 0) * id + (i + 1) * id + (i + 2) * id ... + (i + size) * id
            #   => (i * size + 0 + 1 + 2 ... (size - 1)) * id
            size = page.files[j].size
            checksum += (i * size + ADD_SERIES[size - 1]) * page.files[j].id
            i += size

        i += page.free

    return checksum


TEST_INPUT = """2333133121414131402
"""

PART1_TESTS = [
    (TEST_INPUT, 1928),
]

PART2_TESTS = [
    (TEST_INPUT, 2858),
]
