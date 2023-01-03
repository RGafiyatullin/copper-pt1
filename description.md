We have N parties {P_0, P_1, …, P_N-1} where each party P_i hold a secret value x_i. 
Our objective is to implement an interactive protocol allowing each party to calculate X = x_0 + x_1 + … x_n-1 without any party learn the secret value of any other party. 
The protocol works as follows:
Round 1:
    Each Party P_i:
1.    Generate random values such that x_i = r_0 + … + r_N-1
2.    To every P_j in {P_0, P_1, …, P_N-1} where P_j != P_i, send r_j.
For example, Let N=10 and i=2. Find random values such that x_i = r_0 + r_1 + … + r_9. Send r_0 to P_0, r_1 to P_1 etc. Note that you will be keeping r_2 private to your self.
Round 2:
    Each Party P_i:
1.    Receive the messages from each of P_j in {P_0, P_1, …, P_N-1} where P_j != P_i. 
2.    Compute X = sum of all the random values you received from everyone else + r_i generated in the last round.
3.    Send X to everyone.
Round 3:
    Each Party P_i:
1.    Receive X from everyone else and compare it with the value computed in the last round.
a.    If it is equal, send SUCCESS to everyone. 
b.    If it is not equal, send ABORT to everyone.
Round 4:
    Each Party P_i:
1.    Receive conclusion messages from everyone. 
a.    If no ABORT is received, return X. 
b.    Otherwise, don't return anything.
 