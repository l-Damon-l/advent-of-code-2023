class BoxContents:
    def __init__(self, string: str):
        (self.box_id, self.focal_length) = get_id_and_focal_length(string)
        self.is_remove_op = self._is_remove_op(string)

    def __hash__(self):
        return hash(self.box_id)

    def __eq__(self, other):
        return self.box_id == other.box_id

    @staticmethod
    def _is_remove_op(string: str) -> bool:
        return string.find('-') != -1


class Box:
    def __init__(self, set_id: int):
        self.id = set_id
        self.contents: list[BoxContents] = []

    def handle_box_contents(self, contents: BoxContents):
        if contents.is_remove_op:
            if contents in self.contents:
                self.contents.remove(contents)
            return

        if contents in self.contents:
            index = self.contents.index(contents)
            self.contents[index].focal_length = contents.focal_length
        else:
            self.contents.append(contents)


class Boxes:
    def __init__(self):
        self.boxes: dict[int, Box] = {}

    def handle(self, string: str):
        hash_value = hash_algorithm(string, is_part2=True)
        contents = BoxContents(string)
        if hash_value in self.boxes:
            self.boxes[hash_value].handle_box_contents(contents)
        else:
            self.boxes[hash_value] = Box(hash_value)
            self.boxes[hash_value].handle_box_contents(contents)

    def get_part2_sum(self) -> int:
        sum_val = 0
        for key in self.boxes.keys():
            box_contents = self.boxes[key].contents

            cur_slot = 1
            for contents in box_contents:
                box_id = self.boxes[key].id
                sum_val += (1 + box_id) * cur_slot * contents.focal_length
                cur_slot += 1

        return sum_val


def hash_algorithm(string: str, is_part2: bool = False) -> int:
    value = 0

    # If it's part 2, the operator and focal length are removed
    string = string if not is_part2 else get_id_and_focal_length(string)[0]
    for char in string:
        value += ord(char)
        value = (value * 17) % 256

    return value


def get_id_and_focal_length(string: str) -> (str, int):
    # Get everything before the first dash or equals sign
    op_index = string.find('-') if string.find('-') != -1 else string.find('=')
    str_id = string[0:op_index]

    # Get everything after the first dash or equals sign
    focal_length = string[op_index + 1:]

    # The focal length will be empty if it's a remove operation
    # It is set to -1 in this case, but the number isn't actually used
    focal_length = focal_length if focal_length != '' else '-1'
    return str_id, int(focal_length)


def read_file_into_list(filename: str) -> list[str]:
    with open(filename) as f:
        return f.readline().rstrip().split(',')


class Day11:
    def __init__(self, is_test: bool = False):
        self.data = read_file_into_list('input.txt' if not is_test else 'test_input.txt')

    def part1(self) -> int:
        return sum(map(hash_algorithm, self.data))

    def part2(self) -> int:
        boxes = Boxes()
        for string in self.data:
            boxes.handle(string)

        return boxes.get_part2_sum()


if __name__ == '__main__':
    day11 = Day11()
    print(f"Part 1: {day11.part1()}")
    print(f"Part 2: {day11.part2()}")
