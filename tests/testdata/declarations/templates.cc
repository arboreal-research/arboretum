// Test file for template declarations

template<typename T>
class Container {
public:
    T value;
    void set(T v) { value = v; }
};

template<typename T>
T max_value(T a, T b) {
    return a > b ? a : b;
}

template<>
class Container<int> {
public:
    int value;
    int extra;
};
