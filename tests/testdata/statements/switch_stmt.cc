// Test file for switch statements

void test_switch(int x) {
    switch (x) {
        case 1:
            break;
        case 2:
            break;
        default:
            break;
    }
}

void test_switch_with_fallthrough(int x) {
    switch (x) {
        case 1:
        case 2:
            // handle 1 or 2
            break;
        default:
            break;
    }
}
