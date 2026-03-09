// Test file for function declarations

int add(int a, int b) {
    return a + b;
}

int multiply(int a, int b) {
    return a * b;
}

void no_return() {
    // do nothing
}

int with_default(int x = 10) {
    return x;
}

int constexpr_func() {
    return 42;
}

int template_func() {
    return 0;
}
