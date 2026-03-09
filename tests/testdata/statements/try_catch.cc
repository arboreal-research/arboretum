// Test file for try-catch statements

void test_try_catch() {
    try {
        throw 42;
    } catch (int e) {
        // handle
    }
}

void test_multiple_catch() {
    try {
        throw "error";
    } catch (int e) {
        // handle int
    } catch (const char* e) {
        // handle string
    }
}
