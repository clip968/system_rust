#ifndef __AUTOCXXGEN_H__
#define __AUTOCXXGEN_H__

#include <memory>
#include <string>
#include "cxx.h"
#include <stddef.h>
#ifndef AUTOCXX_NEW_AND_DELETE_PRELUDE
#define AUTOCXX_NEW_AND_DELETE_PRELUDE
// Mechanics to call custom operator new and delete
template <typename T>
auto delete_imp(T *ptr, int) -> decltype((void)T::operator delete(ptr)) {
  T::operator delete(ptr);
}
template <typename T> void delete_imp(T *ptr, long) { ::operator delete(ptr); }
template <typename T> void delete_appropriately(T *obj) {
  // 0 is a better match for the first 'delete_imp' so will match
  // preferentially.
  delete_imp(obj, 0);
}
template <typename T>
auto new_imp(size_t count, int) -> decltype(T::operator new(count)) {
  return T::operator new(count);
}
template <typename T> void *new_imp(size_t count, long) {
  return ::operator new(count);
}
template <typename T> T *new_appropriately() {
  // 0 is a better match for the first 'delete_imp' so will match
  // preferentially.
  return static_cast<T *>(new_imp<T>(sizeof(T), 0));
}
#endif // AUTOCXX_NEW_AND_DELETE_PRELUDE
#include "input.h"



inline std::unique_ptr<std::string> autocxx_make_string_0xe8c1ab2ef1ef4995(::rust::Str str) { return std::make_unique<std::string>(std::string(str)); }
inline Test* Test_autocxx_alloc_autocxx_wrapper_0xe8c1ab2ef1ef4995()  { return new_appropriately<Test>();; }
inline void Test_autocxx_free_autocxx_wrapper_0xe8c1ab2ef1ef4995(Test* arg0)  { delete_appropriately<Test>(arg0);; }
inline std::unique_ptr<std::string> to_string_autocxx_wrapper_0xe8c1ab2ef1ef4995(const Test& autocxx_gen_this)  { return std::make_unique<std::string>(autocxx_gen_this.to_string()); }
inline void new_autocxx_autocxx_wrapper_0xe8c1ab2ef1ef4995(Test* autocxx_gen_this)  { new (autocxx_gen_this) Test(); }
inline void new_synthetic_move_ctor_0xe8c1ab2ef1ef4995_autocxx_wrapper_0xe8c1ab2ef1ef4995(Test* autocxx_gen_this, Test* arg1)  { new (autocxx_gen_this) Test(std::move(*arg1)); }
inline void new_synthetic_const_copy_ctor_0xe8c1ab2ef1ef4995_autocxx_wrapper_0xe8c1ab2ef1ef4995(Test* autocxx_gen_this, const Test& arg1)  { new (autocxx_gen_this) Test(arg1); }
inline void Test_synthetic_destructor_0xe8c1ab2ef1ef4995_autocxx_wrapper_0xe8c1ab2ef1ef4995(Test* arg0)  { arg0->Test::~Test(); }
#endif // __AUTOCXXGEN_H__
