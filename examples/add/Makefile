CC=/opt/riscv/bin/riscv64-unknown-linux-gnu-gcc
OBJCOPY=/opt/riscv/bin/riscv64-unknown-elf-objcopy
SRC=add

all:
#	$(CC) -S $(SRC).c
	$(CC) -Wl,-Ttext=0x0 -nostdlib -march=rv64i -mabi=lp64 -o $(SRC) $(SRC).s
	$(OBJCOPY) -O binary $(SRC) $(SRC).bin

clean:
	rm -rf $(SRC).s $(SRC).bin
