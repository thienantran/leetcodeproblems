class Solution:
    def myAtoi(self, s: str) -> int:
        i = 0
        n = len(s)

        while i < n and s[i] == ' ' : 
            i += 1
        
        if i == n : 
            return 0
        
        sign = 1
        if s[i] == '+' : 
            i += 1
        elif s[i] == '-' : 
            i += 1
            sign = -1
        
        parsed = 0
        while i < n : 
            cur = s[i]
            if not cur.isdigit() : 
                break
            parsed = parsed * 10 + int(cur)
            i += 1
        
        parsed *= sign

        if parsed > 2**31 - 1 : 
            return 2**31 - 1
        elif parsed < -2**31 : 
            return -2**31
        else : 
            return parsed
