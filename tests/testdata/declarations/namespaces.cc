// Test file for namespace declarations

namespace mylib {
    int value = 42;
    
    int function() {
        return 0;
    }
}

namespace detail {
    class InternalClass {
    public:
        void private_method();
    };
}

using namespace mylib;
