# https://leetcode.com/problems/reverse-integer/submissions/
# На вход дается целое число. Если оно 32-битное, вернуть это число отзеркалено, то есть цифры в обратном порядке

def reverse(x):
    st = str(x)
    reversed = []
    intovka = 0
    l = -1
    final = ''
    minus = ''

    for i in range(len(st)):
        reversed.append(st[l])
        l -= 1

    for j in reversed:
        if j == '-':
            minus += j
        final += j

    final = final.replace('-', '')

    intovka += int(minus + final)
    if intovka >= 2 ** 31 or intovka <= -2 ** 31:
        return 0
    return intovka


if __name__ == '__main__':
    print(reverse((-321))) # -123