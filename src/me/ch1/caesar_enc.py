# シーザー暗号
def encrypt(text: str, shift: int) -> str:
    """
    シーザー暗号を行う関数 (各文字をシフトする)

    Args:
        text (str): 暗号化する文字列
        shift(int): シーザー暗号の幅
    
    Returns:
        str: 暗号化された文字列
    """
    code_a = ord("A")
    code_z = ord("Z")
    result = ""
    for ch in text:
        code = ord(ch.upper())
        if code_a <= code <= code_z:
            code = (code - code_a + shift) % 26 + code_a
        result += chr(code)
    return result


enc = encrypt("I Love Python", 3)
dec = encrypt(enc, -3)
print(enc, "=>", dec)
