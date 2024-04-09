; configuration.asm

section .data
    config_data resb 256 ; Buffer to store configuration data

section .text
    global set_configuration
    global get_configuration

set_configuration:
    ; Parameters: 
    ;   Input: config_ptr
    ;   Output: eax = 0 (failure), eax = 1 (success)
    ;   Note: Assumes config_data is a null-terminated string

    ; Get the configuration data pointer from the stack
    mov ebx, [esp + 4] ; config_ptr

    ; Copy the configuration data to the config_data buffer
    mov esi, ebx ; Source pointer
    mov edi, config_data ; Destination pointer
    mov ecx, 256 ; Data length
    rep movsb ; Copy data

    ; For simplicity, we'll just return success
    mov eax, 1 ; Configuration set successfully
    ret

get_configuration:
    ; Parameters: 
    ;   Output: config_ptr
    ;   Note: Assumes config_data is a null-terminated string

    ; Copy the configuration data from the config_data buffer to the config_ptr
    mov esi, config_data ; Source pointer
    mov edi, [esp + 4] ; Destination pointer
    mov ecx, 256 ; Data length
    rep movsb ; Copy data

    ; For simplicity, we'll just return success
    mov eax, 1 ; Configuration retrieved successfully
    ret
