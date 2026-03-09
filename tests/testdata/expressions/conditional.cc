// Test file for conditional expressions

int max(int a, int b) {
    return a > b ? a : b;
}

int abs_val(int x) {
    return x < 0 ? -x : x;
}

int clamp(int x, int min_val, int max_val) {
    return x < min_val ? min_val : (x > max_val ? max_val : x);
}
