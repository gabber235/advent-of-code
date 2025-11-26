from z3 import *


def solve_chronospatial():
    s = Solver()

    # Initial register A (what we're solving for)
    A = BitVec("A", 64)  # Using 64 bits to handle large numbers

    # Constraint: A must be positive
    s.add(A > 0)

    # Target program values
    target = [2, 4, 1, 3, 7, 5, 0, 3, 4, 3, 1, 5, 5, 5, 3, 0]

    # Current state
    curr_A = A

    # For each program value we need to output
    for i, target_val in enumerate(target):
        # Step 1: B = A mod 8 (extract bottom 3 bits)
        B = Extract(2, 0, curr_A)  # 3-bit value

        # Step 2: B = B XOR 3
        B = B ^ BitVecVal(3, 3)  # Make sure XOR uses 3-bit value

        # Step 3: C = A/(2^B)
        # First extend B to 64 bits for the shift
        B_extended = ZeroExt(61, B)  # Extend from 3 to 64 bits
        C = LShR(curr_A, B_extended)

        # Step 4: A = A/8
        curr_A = LShR(curr_A, BitVecVal(3, 64))

        # Step 5: B = B XOR C (need to extract 3 bits from C)
        B = B ^ Extract(2, 0, C)

        # Step 6: B = B XOR 5
        B = B ^ BitVecVal(5, 3)

        # Step 7: Output must match target
        s.add(B == BitVecVal(target_val, 3))

        # If this is the last target value, ensure A will be 0 after the final division
        if i == len(target) - 1:
            s.add(curr_A == 0)

    # Try to solve with optimization for smallest A
    opt = Optimize()
    opt.add(s.assertions())
    opt.minimize(A)

    if opt.check() == sat:
        m = opt.model()
        return m[A].as_long()
    else:
        return None


if __name__ == "__main__":
    result = solve_chronospatial()
    print(f"Solution found: {result}")
