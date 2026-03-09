// Test file for loop statements

void test_for() {
    for (int i = 0; i < 10; i++) {
        // loop body
    }
}

void test_while() {
    int i = 0;
    while (i < 10) {
        i++;
    }
}

void test_do_while() {
    int i = 0;
    do {
        i++;
    } while (i < 10);
}

void test_range_based_for() {
    int arr[] = {1, 2, 3};
    for (int x : arr) {
        // process x
    }
}
