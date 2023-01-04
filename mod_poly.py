import poly_arithmetic as pa

a = pa.Complex(1, 1)
b = pa.Complex(1, 1)

print(a, b)

print(a + b)
print(a * b)

poly_1 = pa.Polynomial([(1,0), (0,2), (1,1)], 3)
poly_2 = pa.Polynomial([(1,1), (1,0), (0,2)], 3)
print(poly_1)
print(poly_1 + poly_2)
print(poly_1 * poly_2)
