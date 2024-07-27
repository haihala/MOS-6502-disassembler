import csv


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

with open('test-bin/mega.bin', 'wb') as f:
    for (opcode, operand_count) in opcodes:
        to_write = [opcode] + operand_count*[255]
        f.write(bytes(to_write))
