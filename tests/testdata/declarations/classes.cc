// Test file for class declarations

class SimpleClass {
public:
    int value;
    void method();
};

class ClassWithMethods {
public:
    int get_value() const { return value; }
    void set_value(int v) { value = v; }
private:
    int value;
};

class InheritedClass : public SimpleClass {
public:
    void extra_method();
};
