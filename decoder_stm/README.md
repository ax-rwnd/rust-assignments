# Decoder STM32F411
Sample code that implements the decoder assignment for the STM32F411 Nucleo MCU.

## Dependencies
* [STM32F411.svd](https://github.com/nikhilkalige/stm32f411/blob/master/svd/STM32F411.svd)
* [svd2rust](https://docs.rs/svd2rust/0.11.4/svd2rust/)
* [xargo](https://github.com/japaric/xargo)
* [rustfmt](https://github.com/rust-lang-nursery/rustfmt)
* [cortex-m-quickstart](https://github.com/japaric/cortex-m-quickstart)

## Usage
1. Install the dependencies
2. Merge this folder with `cortex-m-quickstart`, exclude this file.
3. Generate headers from the .svd: `svd2rust -i STM32F411.svd | rustfmt | tee src/lib.rs` 
4. Build for your bare-metal platform `xargo build --target=thumbv7m-none-eabi`
5. Start up OpenOCD for your target, connect with `arm-none-eabi-gdb` and `continue` to see the amount of cycles used together with the decoded string.

### Counting cycles
Cycle counting is done by enabling the DWT register on the board and reading the cycle counter twice--once before `decode` and once more after. This gives us the amount of cycles required to decode the string.

|Cycles|Release|String|Allocation|
|-----:|:-----:|:-----:|---------|
|174140|No     |DECODE|Heap      |
|11590 |Yes    |DECODE|Heap      |
|4276  |No     |ABC   |Heap      |
|326(!)|Yes    |ABC   |Heap      |
|177032|No     |DECODE|Stack     |
|11986 |Yes    |DECODE|Stack     |
|4353  |No     |ABC   |Stack     |
|338(!)|Yes    |ABC   |Stack     |
