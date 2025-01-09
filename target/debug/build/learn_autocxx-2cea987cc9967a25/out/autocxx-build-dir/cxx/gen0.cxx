#include "input.h"
#include "autocxxgen_ffi.h"
#include <array>
#include <cstddef>
#include <cstdint>
#include <memory>
#include <new>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

namespace rust {
inline namespace cxxbridge1 {
// #include "rust/cxx.h"

namespace {
template <typename T>
class impl;
} // namespace

class String;

#ifndef CXXBRIDGE1_RUST_STR
#define CXXBRIDGE1_RUST_STR
class Str final {
public:
  Str() noexcept;
  Str(const String &) noexcept;
  Str(const std::string &);
  Str(const char *);
  Str(const char *, std::size_t);

  Str &operator=(const Str &) &noexcept = default;

  explicit operator std::string() const;

  const char *data() const noexcept;
  std::size_t size() const noexcept;
  std::size_t length() const noexcept;
  bool empty() const noexcept;

  Str(const Str &) noexcept = default;
  ~Str() noexcept = default;

  using iterator = const char *;
  using const_iterator = const char *;
  const_iterator begin() const noexcept;
  const_iterator end() const noexcept;
  const_iterator cbegin() const noexcept;
  const_iterator cend() const noexcept;

  bool operator==(const Str &) const noexcept;
  bool operator!=(const Str &) const noexcept;
  bool operator<(const Str &) const noexcept;
  bool operator<=(const Str &) const noexcept;
  bool operator>(const Str &) const noexcept;
  bool operator>=(const Str &) const noexcept;

  void swap(Str &) noexcept;

private:
  class uninit;
  Str(uninit) noexcept;
  friend impl<Str>;

  std::array<std::uintptr_t, 2> repr;
};
#endif // CXXBRIDGE1_RUST_STR

#ifndef CXXBRIDGE1_IS_COMPLETE
#define CXXBRIDGE1_IS_COMPLETE
namespace detail {
namespace {
template <typename T, typename = std::size_t>
struct is_complete : std::false_type {};
template <typename T>
struct is_complete<T, decltype(sizeof(T))> : std::true_type {};
} // namespace
} // namespace detail
#endif // CXXBRIDGE1_IS_COMPLETE

namespace detail {
template <typename T, typename = void *>
struct operator_new {
  void *operator()(::std::size_t sz) { return ::operator new(sz); }
};

template <typename T>
struct operator_new<T, decltype(T::operator new(sizeof(T)))> {
  void *operator()(::std::size_t sz) { return T::operator new(sz); }
};
} // namespace detail

template <typename T>
union MaybeUninit {
  T value;
  void *operator new(::std::size_t sz) { return detail::operator_new<T>{}(sz); }
  MaybeUninit() {}
  ~MaybeUninit() {}
};

namespace {
template <typename T>
void destroy(T *ptr) {
  ptr->~T();
}

template <bool> struct deleter_if {
  template <typename T> void operator()(T *) {}
};

template <> struct deleter_if<true> {
  template <typename T> void operator()(T *ptr) { ptr->~T(); }
};
} // namespace
} // namespace cxxbridge1
} // namespace rust

extern "C" {
::std::string *cxxbridge1$autocxx_make_string_0xe8c1ab2ef1ef4995(::rust::Str str_) noexcept {
  ::std::unique_ptr<::std::string> (*autocxx_make_string_0xe8c1ab2ef1ef4995$)(::rust::Str) = ::autocxx_make_string_0xe8c1ab2ef1ef4995;
  return autocxx_make_string_0xe8c1ab2ef1ef4995$(str_).release();
}

::Test *cxxbridge1$Test_autocxx_alloc_autocxx_wrapper_0xe8c1ab2ef1ef4995() noexcept {
  ::Test *(*Test_autocxx_alloc_autocxx_wrapper_0xe8c1ab2ef1ef4995$)() = ::Test_autocxx_alloc_autocxx_wrapper_0xe8c1ab2ef1ef4995;
  return Test_autocxx_alloc_autocxx_wrapper_0xe8c1ab2ef1ef4995$();
}

void cxxbridge1$Test_autocxx_free_autocxx_wrapper_0xe8c1ab2ef1ef4995(::Test *arg0) noexcept {
  void (*Test_autocxx_free_autocxx_wrapper_0xe8c1ab2ef1ef4995$)(::Test *) = ::Test_autocxx_free_autocxx_wrapper_0xe8c1ab2ef1ef4995;
  Test_autocxx_free_autocxx_wrapper_0xe8c1ab2ef1ef4995$(arg0);
}

void cxxbridge1$Test$inc(::Test &self) noexcept {
  void (::Test::*inc$)() = &::Test::inc;
  (self.*inc$)();
}

::std::string *cxxbridge1$to_string_autocxx_wrapper_0xe8c1ab2ef1ef4995(::Test const &autocxx_gen_this) noexcept {
  ::std::unique_ptr<::std::string> (*to_string_autocxx_wrapper_0xe8c1ab2ef1ef4995$)(::Test const &) = ::to_string_autocxx_wrapper_0xe8c1ab2ef1ef4995;
  return to_string_autocxx_wrapper_0xe8c1ab2ef1ef4995$(autocxx_gen_this).release();
}

void cxxbridge1$new_autocxx_autocxx_wrapper_0xe8c1ab2ef1ef4995(::Test *autocxx_gen_this) noexcept {
  void (*new_autocxx_autocxx_wrapper_0xe8c1ab2ef1ef4995$)(::Test *) = ::new_autocxx_autocxx_wrapper_0xe8c1ab2ef1ef4995;
  new_autocxx_autocxx_wrapper_0xe8c1ab2ef1ef4995$(autocxx_gen_this);
}

::std::uint32_t cxxbridge1$x3(::std::uint32_t a) noexcept {
  ::std::uint32_t (*x3$)(::std::uint32_t) = ::x3;
  return x3$(a);
}

void cxxbridge1$new_synthetic_move_ctor_0xe8c1ab2ef1ef4995_autocxx_wrapper_0xe8c1ab2ef1ef4995(::Test *autocxx_gen_this, ::Test *other) noexcept {
  void (*new_synthetic_move_ctor_0xe8c1ab2ef1ef4995_autocxx_wrapper_0xe8c1ab2ef1ef4995$)(::Test *, ::Test *) = ::new_synthetic_move_ctor_0xe8c1ab2ef1ef4995_autocxx_wrapper_0xe8c1ab2ef1ef4995;
  new_synthetic_move_ctor_0xe8c1ab2ef1ef4995_autocxx_wrapper_0xe8c1ab2ef1ef4995$(autocxx_gen_this, other);
}

void cxxbridge1$new_synthetic_const_copy_ctor_0xe8c1ab2ef1ef4995_autocxx_wrapper_0xe8c1ab2ef1ef4995(::Test *autocxx_gen_this, ::Test const &other) noexcept {
  void (*new_synthetic_const_copy_ctor_0xe8c1ab2ef1ef4995_autocxx_wrapper_0xe8c1ab2ef1ef4995$)(::Test *, ::Test const &) = ::new_synthetic_const_copy_ctor_0xe8c1ab2ef1ef4995_autocxx_wrapper_0xe8c1ab2ef1ef4995;
  new_synthetic_const_copy_ctor_0xe8c1ab2ef1ef4995_autocxx_wrapper_0xe8c1ab2ef1ef4995$(autocxx_gen_this, other);
}

void cxxbridge1$Test_synthetic_destructor_0xe8c1ab2ef1ef4995_autocxx_wrapper_0xe8c1ab2ef1ef4995(::Test *autocxx_gen_this) noexcept {
  void (*Test_synthetic_destructor_0xe8c1ab2ef1ef4995_autocxx_wrapper_0xe8c1ab2ef1ef4995$)(::Test *) = ::Test_synthetic_destructor_0xe8c1ab2ef1ef4995_autocxx_wrapper_0xe8c1ab2ef1ef4995;
  Test_synthetic_destructor_0xe8c1ab2ef1ef4995_autocxx_wrapper_0xe8c1ab2ef1ef4995$(autocxx_gen_this);
}

static_assert(::rust::detail::is_complete<::Test>::value, "definition of Test is required");
static_assert(sizeof(::std::unique_ptr<::Test>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::Test>) == alignof(void *), "");
void cxxbridge1$unique_ptr$Test$null(::std::unique_ptr<::Test> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::Test>();
}
::Test *cxxbridge1$unique_ptr$Test$uninit(::std::unique_ptr<::Test> *ptr) noexcept {
  ::Test *uninit = reinterpret_cast<::Test *>(new ::rust::MaybeUninit<::Test>);
  ::new (ptr) ::std::unique_ptr<::Test>(uninit);
  return uninit;
}
void cxxbridge1$unique_ptr$Test$raw(::std::unique_ptr<::Test> *ptr, ::Test *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::Test>(raw);
}
::Test const *cxxbridge1$unique_ptr$Test$get(::std::unique_ptr<::Test> const &ptr) noexcept {
  return ptr.get();
}
::Test *cxxbridge1$unique_ptr$Test$release(::std::unique_ptr<::Test> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$Test$drop(::std::unique_ptr<::Test> *ptr) noexcept {
  ::rust::deleter_if<::rust::detail::is_complete<::Test>::value>{}(ptr);
}

static_assert(sizeof(::std::shared_ptr<::Test>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::shared_ptr<::Test>) == alignof(void *), "");
void cxxbridge1$shared_ptr$Test$null(::std::shared_ptr<::Test> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::Test>();
}
::Test *cxxbridge1$shared_ptr$Test$uninit(::std::shared_ptr<::Test> *ptr) noexcept {
  ::Test *uninit = reinterpret_cast<::Test *>(new ::rust::MaybeUninit<::Test>);
  ::new (ptr) ::std::shared_ptr<::Test>(uninit);
  return uninit;
}
void cxxbridge1$shared_ptr$Test$clone(::std::shared_ptr<::Test> const &self, ::std::shared_ptr<::Test> *ptr) noexcept {
  ::new (ptr) ::std::shared_ptr<::Test>(self);
}
::Test const *cxxbridge1$shared_ptr$Test$get(::std::shared_ptr<::Test> const &self) noexcept {
  return self.get();
}
void cxxbridge1$shared_ptr$Test$drop(::std::shared_ptr<::Test> *self) noexcept {
  self->~shared_ptr();
}

static_assert(sizeof(::std::weak_ptr<::Test>) == 2 * sizeof(void *), "");
static_assert(alignof(::std::weak_ptr<::Test>) == alignof(void *), "");
void cxxbridge1$weak_ptr$Test$null(::std::weak_ptr<::Test> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::Test>();
}
void cxxbridge1$weak_ptr$Test$clone(::std::weak_ptr<::Test> const &self, ::std::weak_ptr<::Test> *ptr) noexcept {
  ::new (ptr) ::std::weak_ptr<::Test>(self);
}
void cxxbridge1$weak_ptr$Test$downgrade(::std::shared_ptr<::Test> const &shared, ::std::weak_ptr<::Test> *weak) noexcept {
  ::new (weak) ::std::weak_ptr<::Test>(shared);
}
void cxxbridge1$weak_ptr$Test$upgrade(::std::weak_ptr<::Test> const &weak, ::std::shared_ptr<::Test> *shared) noexcept {
  ::new (shared) ::std::shared_ptr<::Test>(weak.lock());
}
void cxxbridge1$weak_ptr$Test$drop(::std::weak_ptr<::Test> *self) noexcept {
  self->~weak_ptr();
}

::std::vector<::Test> *cxxbridge1$std$vector$Test$new() noexcept {
  return new ::std::vector<::Test>();
}
::std::size_t cxxbridge1$std$vector$Test$size(::std::vector<::Test> const &s) noexcept {
  return s.size();
}
::Test *cxxbridge1$std$vector$Test$get_unchecked(::std::vector<::Test> *s, ::std::size_t pos) noexcept {
  return &(*s)[pos];
}
void cxxbridge1$std$vector$Test$push_back(::std::vector<::Test> *v, ::Test *value) noexcept {
  v->push_back(::std::move(*value));
  ::rust::destroy(value);
}
void cxxbridge1$std$vector$Test$pop_back(::std::vector<::Test> *v, ::Test *out) noexcept {
  ::new (out) ::Test(::std::move(v->back()));
  v->pop_back();
}
static_assert(sizeof(::std::unique_ptr<::std::vector<::Test>>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<::std::vector<::Test>>) == alignof(void *), "");
void cxxbridge1$unique_ptr$std$vector$Test$null(::std::unique_ptr<::std::vector<::Test>> *ptr) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::Test>>();
}
void cxxbridge1$unique_ptr$std$vector$Test$raw(::std::unique_ptr<::std::vector<::Test>> *ptr, ::std::vector<::Test> *raw) noexcept {
  ::new (ptr) ::std::unique_ptr<::std::vector<::Test>>(raw);
}
::std::vector<::Test> const *cxxbridge1$unique_ptr$std$vector$Test$get(::std::unique_ptr<::std::vector<::Test>> const &ptr) noexcept {
  return ptr.get();
}
::std::vector<::Test> *cxxbridge1$unique_ptr$std$vector$Test$release(::std::unique_ptr<::std::vector<::Test>> &ptr) noexcept {
  return ptr.release();
}
void cxxbridge1$unique_ptr$std$vector$Test$drop(::std::unique_ptr<::std::vector<::Test>> *ptr) noexcept {
  ptr->~unique_ptr();
}
} // extern "C"
