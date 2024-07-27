import csv
import sys


def operand_len(address_mode):
    if address_mode in ["A", "impl"]:
        return 0

    if address_mode in ["abs", "abs,X", "abs,Y", "ind"]:
        return 2

    return 1


opcodes = []

with open('opcode-table.csv') as csvfile:
    spamreader = csv.reader(csvfile, delimiter=';', quotechar='|')
    for row, items in enumerate(spamreader):
        for column, item in enumerate(items):
            if item == '---' or len(item) == 0:
                continue

            opcodes.append(
                (
                    row*16 + column,
                    operand_len(item.split(" ", 1)[1])
                )
            )

giga_mode = len(sys.argv) > 1 and sys.argv[1] == "giga"

operand_values = [255] if not giga_mode else [i for i in range(0, 256)]


with open(f'test-bin/{"giga" if giga_mode else "mega"}.bin', 'wb') as f:
    for operand in operand_values:
        for (opcode, operand_count) in opcodes:
            to_write = [opcode] + operand_count*[operand]
            f.write(bytes(to_write))
