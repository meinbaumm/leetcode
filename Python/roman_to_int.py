# https://leetcode.com/problems/roman-to-integer/
# Получить римскую цифру, вернуть арабскую
dada = {'I':1,
        'V':5,
        'X':10,
        'L':50,
        'C':100,
        'D':500,
        'M':1000,
        'IV':4,
        'IX':9,
        'XL':40,
        'XC':90,
        'CD':400,
        'CM':900}

def romanToInt(s):

    if len(s) == 2:
        try:
            n = dada[s]
            return n
        except:
            return dada[s[0]] + dada[s[1]]

    if len(s) == 1:
        return dada[s]

    i = 0
    numb = 0
    while i < len(s):

        try:
            x = dada[s[i:i + 2]]
            numb += x
            i += 2
        except:
            x = dada[s[i]]
            numb += x
            i += 1

    return numb

if __name__ == '__main__':
    print(romanToInt('XVI'))