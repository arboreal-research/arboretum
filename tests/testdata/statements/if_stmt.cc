// Test file for if statement handling

void test_if(int x) {
    if (x > 0) {
        // positive
    }
}

void test_if_else(int x) {
    if (x > 0) {
        // positive
    } else {
        // non-positive
    }
}

void test_if_elseif(int x) {
    if (x > 0) {
        // positive
    } else if (x < 0) {
        // negative
    } else {
        // zero
    }
}

void test_nested_if(int x, int y) {
    if (x > 0) {
        if (y > 0) {
            // both positive
        }
    }
}
