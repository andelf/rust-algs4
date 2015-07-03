"""
(seed = 932025)
Suppose that you time a program as a function of N and produce
the following table.

        N   seconds
-------------------
    16384     0.000
    32768     0.000
    65536     0.001
   131072     0.004
   262144     0.009
   524288     0.025
  1048576     0.068
  2097152     0.181
  4194304     0.482
  8388608     1.287
 16777216     3.436
 33554432     9.171
 67108864    24.490
134217728    65.380
268435456   174.542
536870912   465.976


Estimate the order of growth of the running time as a function of N.
Assume that the running time obeys a power law T(N) ~ a N^b. For your
answer, enter the constant b. Your answer will be marked as correct
if it is within 1% of the target answer - we recommend using
two digits after the decimal separator, e.g., 2.34.
"""
from pylab import *


data = """    16384     0.000
    32768     0.000
    65536     0.001
   131072     0.004
   262144     0.009
   524288     0.025
  1048576     0.068
  2097152     0.181
  4194304     0.482
  8388608     1.287
 16777216     3.436
 33554432     9.171
 67108864    24.490
134217728    65.380
268435456   174.542
536870912   465.976"""

vals = data.split()

N = array(map(int, vals[::2]))
T = array(map(float, vals[1::2]))

from scipy.optimize import curve_fit

def func(x, a, b, c):
    return a * np.power(x, b) + c

popt, pcov = curve_fit(func, N, T)

a, b, c = popt

print "b =", b

plot(N, T, 'r.')
plot(N, a*N**b +c )

show()
