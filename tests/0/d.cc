#include <vector>

template <typename T, typename U> class Animal {
public:
  Animal();

  Animal(T t) : value_t(t) {}

  ~Animal() = default;

  void pet() {}

  T value_t;
  U value_u;
};

template <typename T, typename U> Animal<T, U>::Animal() {}

using AnimalInt = Animal<int, int>;

template <typename T> class Animal<T, double> {
  void bark() {}
};

// Function Template
template <typename U, typename V> V f(U b) {
  AnimalInt animal;
  animal.pet();
  return b;
}

// Function Template Specialization
template <> double f<float, double>(float b) { return b; }

void force_uses() {
  f<char, char>('a');
  Animal<double, double> animal;
}