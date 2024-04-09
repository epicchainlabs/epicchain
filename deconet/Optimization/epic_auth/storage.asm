; storage.asm

section .data
    storage_data resb 256 ; Buffer to store data

section .text
    global store_data
    global retrieve_data

store_data:
    ; Parameters: 
    ;   Input: data_ptr
    ;   Output: eax = 0 (failure), eax = 1 (success)
    ;   Note: Assumes data is a null-terminated string

    ; Get the data pointer from the stack
    mov ebx, [esp + 4] ; data_ptr

    ; Copy the data to the storage buffer
    mov esi, ebx ; Source pointer
    mov edi, storage_data ; Destination pointer
    mov ecx, 256 ; Data length
    rep movsb ; Copy data

    ; For simplicity, we'll just return success
    mov eax, 1 ; Data stored successfully
    ret

retrieve_data:
    ; Parameters: 
    ;   Output: data_ptr
    ;   Note: Assumes data is a null-terminated string

    ; Copy the data from the storage buffer to the data pointer
    mov esi, storage_data ; Source pointer
    mov edi, [esp + 4] ; Destination pointer
    mov ecx, 256 ; Data length
    rep movsb ; Copy data

    ; For simplicity, we'll just return success
    mov eax, 1 ; Data retrieved successfully
    ret
