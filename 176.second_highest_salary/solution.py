import pandas as pd

def second_highest_salary(employee: pd.DataFrame) -> pd.DataFrame:
    d=[]
    for i in employee["salary"]:
        d.append(i)
    d.sort()
    if(len(d)<=1):
        e={"SecondHighestSalary":[None]}
    elif(d[-1]==d[-2] and len(d)==2):
        e={"SecondHighestSalary":[None]}
    elif(d[-1]==d[-2] and len(d)>2):
        e={"SecondHighestSalary":[d[-3]]}
    else:
        e={"SecondHighestSalary":[d[-2]]}
    c=pd.DataFrame(e)
    return c
    
