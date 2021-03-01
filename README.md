# Specialized Dynamic Binary Translator
A research project exploring ARM64-to-ARM64 binary translation that allows machine code to be instrumented during runtime based on user specifications.

# What is Binary Translation
Binary translation is a technology commonly commonly seen in virtual machines and emulators, where a program has its machine code taken and modified. It is typically used to translate programs across architectures such as ARM and x86, and can also involve anything from replacing OS-specific system calls and allowing Windows programs to run on Linux, to enabling a user to emulate a smartphone on their desktop computer. 

# Translating architectures into themselves
Translating ARM64 into ARM64 itself offers a unique advantage—namely, the ability to modify a program’s behavior and code. The project plans to use such binary translation on programs during their runtime, and to do so at a much larger scale than what is currently available. Every single instruction of a program will be copied into memory, automatically changed to whatever a user wants it to be if they wish, and will then be executed. 

# Binary Translators as Hypervisors
Because the user will have full control over what code is executed, both now and in the future, such a tool can in effect be used as a hypervisor—a platform under which virtual machines and software can be run. Any possible state of the computer can be emulated by modifying instructions, their values, or the address in memory where they are loaded. Furthermore, this can all be done without any detectable side effects and the program ever realizing that it’s being run under a virtual machine, thus providing a strong form of isolation.

# Applications
Binary-translation-based virtualization would create new debugging methods through code instrumentation, which would work even in advanced cases such as self-modifying code, allowing for the analysis of polymorphic viruses. Additionally, it will provide a simple way to run applications in security-enhanced sandboxes without degrading performance as most of the instructions will be executed directly. This can even be done on platforms which don't support hardware virtualization—the project will be run in user mode at all times, and does not require elevated privilege to function.

# Roadmap
Completed:
- Fully parse ARM documentation provided in XML format.

In Progress:
- Generate code that can automatically identify, decode, and encode different opcodes of ARM64 ISA.

To Do:
- Assemble generated code into a standalone library that can be switched out if needed.
- Create and test a simple specification file format that users will give.
- Design a data structure to efficiently store information about all translated basic blocks.
- Read and translate basic blocks of program data to memory.
- Fix relative address jumps.
- Permanently assign a register to point at the allocated region, but load expected data into the register whenever the program uses it.
- Re-read and re-translate basic blocks whenever the program will write to memory at their address.
- Fix direct address jumps.
- Test and ensure the functionality of the specifications file format.
- Log and display useful information and statistics to the user such as addresses where the program wrote.
- Improve runtime efficiency of instruction encoding and decoding.
- Support for other architectures such as ARM32, x86,and x86-64
