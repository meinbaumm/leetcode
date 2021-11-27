# https://leetcode.com/problems/defanging-an-ip-address/
# Defanging an IP Address

def defangIPaddr(address):
    return address.replace(".", "[.]")


address = "1.1.1.1"


if __name__ == '__main__':
    print(defangIPaddr(address))