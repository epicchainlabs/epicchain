; visualize_data.asm

section .data
    data_to_visualize db 256 dup(0) ; Buffer to store the data to visualize
    visualization_result db 512 dup(0) ; Buffer to store the visualization result (each byte visualized as two ASCII characters)
    error_message db "Error: Unable to visualize data", 0 ; Error message

section .text
    global visualize_data

; Function: visualize_data
; Description: Visualizes the input data and returns the visualization result, or an error message if visualization fails.
; Parameters: None
; Returns: Pointer to the visualization result, or pointer to the error message if visualization fails
visualize_data:
    ; Initialize registers
    xor esi, esi ; Clear source index
    mov edi, visualization_result ; Destination pointer
    xor ecx, ecx ; Clear ECX register for loop counter

    ; Loop to visualize data
visualize_data_loop:
    ; Check if input data is valid
    cmp byte [data_to_visualize + esi], 0 ; Check if byte is null terminator
    je visualize_data_done ; Exit loop if null terminator is found

    ; Perform data visualization (convert each byte to two ASCII characters)
    mov al, [data_to_visualize + esi] ; Load byte from source
    mov ah, al ; Copy byte for second ASCII character
    shr al, 4 ; Shift 4 bits to the right to get the first ASCII character
    call convert_to_ascii ; Convert to ASCII character
    mov [edi + ecx], al ; Store first ASCII character
    inc ecx ; Increment counter
    and ah, 0x0F ; Mask upper 4 bits to get the second ASCII character
    call convert_to_ascii ; Convert to ASCII character
    mov [edi + ecx], ah ; Store second ASCII character
    inc ecx ; Increment counter
    inc esi ; Move to next byte
    jmp visualize_data_loop ; Continue loop

visualize_data_done:
    ; Return the pointer to the visualization result
    mov eax, visualization_result
    ret

; Function: convert_to_ascii
; Description: Converts a 4-bit value to its ASCII representation ('0'-'9' or 'A'-'F').
; Parameters: AL - 4-bit value to convert
; Returns: AL - ASCII character
convert_to_ascii:
    add al, '0' ; Add ASCII offset for digits '0'-'9'
    cmp al, '9' ; Check if character is a digit
    jbe convert_to_ascii_done ; Jump if it's a digit
    add al, 'A' - '9' - 1 ; Adjust for letters 'A'-'F'
convert_to_ascii_done:
    ret

