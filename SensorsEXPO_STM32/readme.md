# Initial Setup

## VS Code


## Setting up Docker


Objective: Setup a Docker Container
Problem(s): Not able to setup Docker Desktop
Solution: Setup apt repository

https://docs.docker.com/engine/install/ubuntu/#install-using-the-repository

1. Clear out all the previous unofficial packages.
    
    `docker.io
    docker-compose
    docker-compose-v2
    docker-doc
    podman-docker`

    `$ for pkg in docker.io docker-doc docker-compose docker-compose-v2 podman-docker containerd runc; do sudo apt-get remove $pkg; done`
2. Install official GPG key & Apt Source

    ### Add Docker's official GPG key:
    sudo apt-get update
    sudo apt-get install ca-certificates curl
    sudo install -m 0755 -d /etc/apt/keyrings
    sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
    sudo chmod a+r /etc/apt/keyrings/docker.asc

    ### Add the repository to Apt sources:
    echo \
    "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
    $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
    sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
    sudo apt-get update
3. Install Docker Packages
    `sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin`
4. Hello World

    *Start Docker container* 
    `systemctl start docker`

    Run command
    `sudo docker run hello-world`

        [sudo] password for vboxuser: 

        Hello from Docker!
        This message shows that your installation appears to be working correctly.

        To generate this message, Docker took the following steps:
        1. The Docker client contacted the Docker daemon.
        2. The Docker daemon pulled the "hello-world" image from the Docker Hub.
            (amd64)
        3. The Docker daemon created a new container from that image which runs the
            executable that produces the output you are currently reading.
        4. The Docker daemon streamed that output to the Docker client, which sent it
            to your terminal.

        To try something more ambitious, you can run an Ubuntu container with:
        $ docker run -it ubuntu bash

        Share images, automate workflows, and more with a free Docker ID:
        https://hub.docker.com/

        For more examples and ideas, visit:
        https://docs.docker.com/get-started/

# Lab Exercise 1 – Containerize your Rust Environment

Notes: Ensure you have watched and/or attended the Session 1 presentation.
Overview:
In this lab, you will set up the Rust environment on your machine inside of a Docker container.
Lab Instructions:
For this lab, you’ll need your PC and an internet connection. You won’t need your development board quite yet. The goal is to have a containerized development environment that you can use to build Rust applications.
The major steps that you will take to accomplish this includes:

1) Creating a Docker Container

2) Verify your Docker Container with Rust

3) Create a “Hello World!” application

4) Modify “Hello World!” to become familiar with Rust functions

./svd2rustU5/devManager.sh docker_image
Building the Docker image...
ERROR: permission denied while trying to connect to the Docker daemon socket at unix:///var/run/docker.sock: Head "http://%2Fvar%2Frun%2Fdocker.sock/_ping": dial unix /var/run/do
cker.sock: connect: permission denied                                                                                                                                             Docker image built successfully.



# Lab Exercise 2 – Building a Rust PAC and Hello Blinky Application
    Overview:
    In this lab, you will develop a blinky LED application for the STM32U575 Nucleo board. The lab will require you to develop your own peripheral access crate (PAC). You’ll also need to design and implement a blinky application in the Rust language.
    Lab Instructions:
    For this lab, there are several major steps that you will need to work through to blink an LED on the development board. These steps include:
    1) Creating the STM32U575 PAC

    2) Creating a blinky application from an embedded application template
    
    3) Updating the template for the STM32U575
    
    4) Writing the blinky application
   
    5) Configuring debug scripts and launch configurations
   
    Let’s get started!



## Building your PAC
    root@330b454f3f6e:/home/app/stm32u575_pac# cargo build -r
    Updating crates.io index
     Locking 19 packages to latest compatible versions
      Adding bare-metal v0.2.5 (latest: v1.0.0)
      Adding bitfield v0.13.2 (latest: v0.15.0)
      Adding cortex-m v0.7.7
      Adding cortex-m-rt v0.7.3
      Adding cortex-m-rt-macros v0.7.0
      Adding critical-section v1.1.2
      Adding embedded-hal v0.2.7 (latest: v1.0.0)
      Adding nb v0.1.3 (latest: v1.1.0)
      Adding nb v1.1.0
      Adding proc-macro2 v1.0.86
      Adding quote v1.0.36
      Adding rustc_version v0.2.3 (latest: v0.4.0)
      Adding semver v0.9.0 (latest: v1.0.23)
      Adding semver-parser v0.7.0 (latest: v0.10.2)
      Adding syn v1.0.109 (latest: v2.0.69)
      Adding unicode-ident v1.0.12
      Adding vcell v0.1.3
      Adding void v1.0.2
      Adding volatile-register v0.2.2
  Downloaded cortex-m-rt-macros v0.7.0
  Downloaded void v1.0.2
  Downloaded semver-parser v0.7.0
  Downloaded rustc_version v0.2.3
  Downloaded nb v0.1.3
  Downloaded vcell v0.1.3
  Downloaded embedded-hal v0.2.7
  Downloaded bitfield v0.13.2
  Downloaded volatile-register v0.2.2
  Downloaded bare-metal v0.2.5
  Downloaded semver v0.9.0
  Downloaded nb v1.1.0
  Downloaded cortex-m-rt v0.7.3
  Downloaded cortex-m v0.7.7
  Downloaded 14 crates (322.5 KB) in 0.33s
   Compiling semver-parser v0.7.0
   Compiling proc-macro2 v1.0.86
   Compiling unicode-ident v1.0.12
   Compiling syn v1.0.109
   Compiling nb v1.1.0
   Compiling cortex-m v0.7.7
   Compiling semver v0.9.0
   Compiling cortex-m-rt v0.7.3
   Compiling nb v0.1.3
   Compiling rustc_version v0.2.3
   Compiling vcell v0.1.3
   Compiling void v1.0.2
   Compiling volatile-register v0.2.2
   Compiling bitfield v0.13.2
   Compiling embedded-hal v0.2.7
   Compiling bare-metal v0.2.5
   Compiling stm32u575_pac v0.1.0 (/home/app/stm32u575_pac)
   Compiling quote v1.0.36
   Compiling cortex-m-rt-macros v0.7.0
    Finished `release` profile [optimized] target(s) in 20.89s




    root@25c9e3605139:/home/app/blinky# cargo size --bin blinky --release -- -A
   Compiling semver-parser v0.7.0
   Compiling proc-macro2 v1.0.86
   Compiling cortex-m v0.7.7
   Compiling unicode-ident v1.0.12
   Compiling nb v1.1.0
   Compiling semver v0.9.0
   Compiling rustc_version v0.2.3
   Compiling syn v1.0.109
   Compiling cortex-m-rt v0.7.3
   Compiling bare-metal v0.2.5
   Compiling nb v0.1.3
   Compiling vcell v0.1.3
   Compiling void v1.0.2
   Compiling embedded-hal v0.2.7
   Compiling volatile-register v0.2.2
   Compiling bitfield v0.13.2
   Compiling quote v1.0.36
   Compiling critical-section v1.1.2
   Compiling stm32u575_pac v0.1.0 (/home/app/stm32u575_pac)
   Compiling cortex-m-semihosting v0.3.7
   Compiling blinky v0.1.0 (/home/app/blinky)
   Compiling panic-halt v0.2.0
   Compiling cortex-m-rt-macros v0.7.0
    Finished `release` profile [optimized + debuginfo] target(s) in 18.74s
blinky  :
section              size        addr
.vector_table         600   0x8000000
.text                 444   0x8000258
.rodata                 0   0x8000414
.data                   0  0x20000000
.gnu.sgstubs            0   0x8000420
.bss                    4  0x20000000
.uninit                 0  0x20000004
.debug_loc            654         0x0
.debug_abbrev        1961         0x0
.debug_info         22735         0x0
.debug_aranges        632         0x0
.debug_ranges        1240         0x0
.debug_str          18422         0x0
.comment               64         0x0
.ARM.attributes        58         0x0
.debug_frame          964         0x0
.debug_line          3733         0x0
.debug_pubnames       803         0x0
.debug_pubtypes        71         0x0
Total               52385


root@25c9e3605139:/home/app/blinky# cargo objdump --bin blinky --release -- --disassemble --no-show-raw-insn --print-imm-hex
    Finished `release` profile [optimized + debuginfo] target(s) in 0.02s

blinky: file format elf32-littlearm

Disassembly of section .text:

08000258 <__stext>:
 8000258:       bl      0x80003e6 <__pre_init>  @ imm = #0x18a
 800025c:       ldr     r0, [pc, #0x38]         @ 0x8000298 <__stext+0x40>
 800025e:       ldr     r1, [pc, #0x3c]         @ 0x800029c <__stext+0x44>
 8000260:       movs    r2, #0x0
 8000262:       cmp     r1, r0
 8000264:       beq     0x800026a <__stext+0x12> @ imm = #0x2
 8000266:       stm     r0!, {r2}
 8000268:       b       0x8000262 <__stext+0xa> @ imm = #-0xa
 800026a:       ldr     r0, [pc, #0x34]         @ 0x80002a0 <__stext+0x48>
 800026c:       ldr     r1, [pc, #0x34]         @ 0x80002a4 <__stext+0x4c>
 800026e:       ldr     r2, [pc, #0x38]         @ 0x80002a8 <__stext+0x50>
 8000270:       cmp     r1, r0
 8000272:       beq     0x800027a <__stext+0x22> @ imm = #0x4
 8000274:       ldm     r2!, {r3}
 8000276:       stm     r0!, {r3}
 8000278:       b       0x8000270 <__stext+0x18> @ imm = #-0xc
 800027a:       ldr     r0, [pc, #0x30]         @ 0x80002ac <__stext+0x54>
 800027c:       mov.w   r1, #0xf00000
 8000280:       ldr     r2, [r0]
 8000282:       orr.w   r2, r2, r1
 8000286:       str     r2, [r0]
 8000288:       dsb     sy
 800028c:       isb     sy
 8000290:       bl      0x80002b0 <main>        @ imm = #0x1c
 8000294:       udf     #0x0
 8000296:       movs    r0, r0
 8000298: 00 00 00 20   .word   0x20000000
 800029c: 04 00 00 20   .word   0x20000004
 80002a0: 00 00 00 20   .word   0x20000000
 80002a4: 00 00 00 20   .word   0x20000000
 80002a8: 14 04 00 08   .word   0x08000414
 80002ac: 88 ed 00 e0   .word   0xe000ed88

080002b0 <main>:
 80002b0:       push    {r7, lr}
 80002b2:       mov     r7, sp
 80002b4:       bl      0x80002b8 <blinky::__cortex_m_rt_main::h67605acbf3aa0ba9> @ imm = #0x0

080002b8 <blinky::__cortex_m_rt_main::h67605acbf3aa0ba9>:
 80002b8:       push    {r7, lr}
 80002ba:       mov     r7, sp
 80002bc:       bl      0x80003f2 <__primask_r> @ imm = #0x132
 80002c0:       mov     r4, r0
 80002c2:       bl      0x80003ea <__cpsid>     @ imm = #0x124
 80002c6:       movw    r5, #0x0
 80002ca:       movt    r5, #0x2000
 80002ce:       ldrb    r0, [r5, #0x1]
 80002d0:       cmp     r0, #0x0
 80002d2:       bne     0x80003b6 <blinky::__cortex_m_rt_main::h67605acbf3aa0ba9+0xfe> @ imm = #0xe0
 80002d4:       movs    r0, #0x1
 80002d6:       strb    r0, [r5, #0x1]
 80002d8:       lsls    r0, r4, #0x1f
 80002da:       it      eq
 80002dc:       bleq    0x80003ee <__cpsie>     @ imm = #0x10e
 80002e0:       bl      0x80003f2 <__primask_r> @ imm = #0x10e
 80002e4:       mov     r4, r0
 80002e6:       bl      0x80003ea <__cpsid>     @ imm = #0x100
 80002ea:       ldrb    r6, [r5]
 80002ec:       cmp     r6, #0x0
 80002ee:       itt     eq
 80002f0:       moveq   r0, #0x1
 80002f2:       strbeq  r0, [r5]
 80002f4:       lsls    r0, r4, #0x1f
 80002f6:       it      eq
 80002f8:       bleq    0x80003ee <__cpsie>     @ imm = #0xf2
 80002fc:       cmp     r6, #0x0
 80002fe:       bne     0x80003be <blinky::__cortex_m_rt_main::h67605acbf3aa0ba9+0x106> @ imm = #0xbc
 8000300:       movw    r0, #0xc8c
 8000304:       movs    r2, #0x1
 8000306:       movt    r0, #0x4602
 800030a:       movs    r3, #0x0
 800030c:       ldr     r1, [r0]
 800030e:       orr     r1, r1, #0x46
 8000312:       str     r1, [r0]
 8000314:       movw    r0, #0x400
 8000318:       movt    r0, #0x4202
 800031c:       ldr     r1, [r0]
 800031e:       bfi     r1, r2, #14, #2
 8000322:       str     r1, [r0]
 8000324:       ldr.w   r1, [r0, #0x400]
 8000328:       bfi     r1, r2, #14, #2
 800032c:       str.w   r1, [r0, #0x400]
 8000330:       movw    r0, #0x1800
 8000334:       movt    r0, #0x4202
 8000338:       ldr     r1, [r0]
 800033a:       bfi     r1, r2, #4, #2
 800033e:       str     r1, [r0]
 8000340:       movw    r0, #0x7100
 8000344:       movw    r1, #0x814
 8000348:       movw    r2, #0xe010
 800034c:       movt    r0, #0x2
 8000350:       movt    r1, #0x4202
 8000354:       movt    r2, #0xe000
 8000358:       ldr     r6, [r1]
 800035a:       orr     r6, r6, #0x80
 800035e:       str     r6, [r1]
 8000360:       str     r0, [r2, #0x4]
 8000362:       str     r3, [r2, #0x8]
 8000364:       ldr     r6, [r2]
 8000366:       orr     r6, r6, #0x1
 800036a:       str     r6, [r2]
 800036c:       ldr     r6, [r2]
 800036e:       lsls    r6, r6, #0xf
 8000370:       bmi     0x8000386 <blinky::__cortex_m_rt_main::h67605acbf3aa0ba9+0xce> @ imm = #0x12
 8000372:       ldr     r6, [r2]
 8000374:       lsls    r6, r6, #0xf
 8000376:       itt     pl
 8000378:       ldrpl   r6, [r2]
 800037a:       lslspl.w        r6, r6, #0xf
 800037e:       bmi     0x8000386 <blinky::__cortex_m_rt_main::h67605acbf3aa0ba9+0xce> @ imm = #0x4
 8000380:       ldr     r6, [r2]
 8000382:       lsls    r6, r6, #0xf
 8000384:       bpl     0x800036c <blinky::__cortex_m_rt_main::h67605acbf3aa0ba9+0xb4> @ imm = #-0x1c
 8000386:       ldr     r6, [r1]
 8000388:       bic     r6, r6, #0x80
 800038c:       str     r6, [r1]
 800038e:       str     r0, [r2, #0x4]
 8000390:       str     r3, [r2, #0x8]
 8000392:       ldr     r6, [r2]
 8000394:       orr     r6, r6, #0x1
 8000398:       str     r6, [r2]
 800039a:       ldr     r6, [r2]
 800039c:       lsls    r6, r6, #0xf
 800039e:       bmi     0x8000358 <blinky::__cortex_m_rt_main::h67605acbf3aa0ba9+0xa0> @ imm = #-0x4a
 80003a0:       ldr     r6, [r2]
 80003a2:       lsls    r6, r6, #0xf
 80003a4:       itt     pl
 80003a6:       ldrpl   r6, [r2]
 80003a8:       lslspl.w        r6, r6, #0xf
 80003ac:       bmi     0x8000358 <blinky::__cortex_m_rt_main::h67605acbf3aa0ba9+0xa0> @ imm = #-0x58
 80003ae:       ldr     r6, [r2]
 80003b0:       lsls    r6, r6, #0xf
 80003b2:       bpl     0x800039a <blinky::__cortex_m_rt_main::h67605acbf3aa0ba9+0xe2> @ imm = #-0x1c
 80003b4:       b       0x8000358 <blinky::__cortex_m_rt_main::h67605acbf3aa0ba9+0xa0> @ imm = #-0x60
 80003b6:       movs    r0, #0x1
 80003b8:       bics    r0, r4
 80003ba:       bl      0x80003da <_critical_section_1_0_release> @ imm = #0x1c
 80003be:       bl      0x80003d2 <core::option::unwrap_failed::hcc2235ec2fb7423e> @ imm = #0x10

080003c2 <core::panicking::panic_fmt::h8561f936ab47e5f8>:
 80003c2:       push    {r7, lr}
 80003c4:       mov     r7, sp
 80003c6:       bl      0x80003e8 <rust_begin_unwind> @ imm = #0x1e

080003ca <core::panicking::panic::hd6724d5ad61dc150>:
 80003ca:       push    {r7, lr}
 80003cc:       mov     r7, sp
 80003ce:       bl      0x80003c2 <core::panicking::panic_fmt::h8561f936ab47e5f8> @ imm = #-0x10

080003d2 <core::option::unwrap_failed::hcc2235ec2fb7423e>:
 80003d2:       push    {r7, lr}
 80003d4:       mov     r7, sp
 80003d6:       bl      0x80003ca <core::panicking::panic::hd6724d5ad61dc150> @ imm = #-0x10

080003da <_critical_section_1_0_release>:
 80003da:       cmp     r0, #0x0
 80003dc:       it      eq
 80003de:       bxeq    lr
 80003e0:       b.w     0x80003ee <__cpsie>     @ imm = #0xa

080003e4 <WWDG>:
 80003e4:       b       0x80003e4 <WWDG>        @ imm = #-0x4

080003e6 <__pre_init>:
 80003e6:       bx      lr

080003e8 <rust_begin_unwind>:
 80003e8:       b       0x80003e8 <rust_begin_unwind> @ imm = #-0x4

080003ea <__cpsid>:
 80003ea:       cpsid i
 80003ec:       bx      lr

080003ee <__cpsie>:
 80003ee:       cpsie i
 80003f0:       bx      lr

080003f2 <__primask_r>:
 80003f2:       mrs     r0, primask
 80003f6:       bx      lr

080003f8 <HardFaultTrampoline>:
 80003f8:       mov     r0, lr
 80003fa:       movs    r1, #0x4
 80003fc:       tst     r0, r1
 80003fe:       bne     0x8000408 <HardFaultTrampoline+0x10> @ imm = #0x6
 8000400:       mrs     r0, msp
 8000404:       b.w     0x8000410 <HardFault_>  @ imm = #0x8
 8000408:       mrs     r0, psp
 800040c:       b.w     0x8000410 <HardFault_>  @ imm = #0x0

08000410 <HardFault_>:
 8000410:       b       0x8000410 <HardFault_>  @ imm = #-0x4
 8000412:       bmi     0x80003be <blinky::__cortex_m_rt_main::h67605acbf3aa0ba9+0x106> @ imm = #-0x58
root@25c9e3605139:/home/app/blinky# cargo readobj --bin blinky -- --file-headers
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
ELF Header:
  Magic:   7f 45 4c 46 01 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF32
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              EXEC (Executable file)
  Machine:                           ARM
  Version:                           0x1
  Entry point address:               0x8000259
  Start of program headers:          52 (bytes into file)
  Start of section headers:          868328 (bytes into file)
  Flags:                             0x5000400
  Size of this header:               52 (bytes)
  Size of program headers:           32 (bytes)
  Number of program headers:         5
  Size of section headers:           40 (bytes)
  Number of section headers:         23
  Section header string table index: 21

  