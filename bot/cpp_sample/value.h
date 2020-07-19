#pragma once

#include <cassert>
#include <map>
#include <memory>
#include <string>
#include <vector>

enum class ValueKind {
  Number,
  Nil,
  Cons,
};

class Value {
public:
  Value(int64_t num)
      : kind_(ValueKind::Number), number_(num), car_(nullptr), cdr_(nullptr) {}
  Value() : kind_(ValueKind::Nil), number_(0), car_(nullptr), cdr_(nullptr) {}
  Value(const Value &v)
      : kind_(v.kind_), number_(v.number_), car_(v.car_), cdr_(v.cdr_) {}
  Value(Value *car, Value *cdr)
      : kind_(ValueKind::Cons), number_(0), car_(new Value(*car)),
        cdr_(new Value(*cdr)) {}
  Value(Value car, Value cdr)
      : kind_(ValueKind::Cons), number_(0), car_(new Value(car)),
        cdr_(new Value(cdr)) {}
  Value(const std::vector<Value> &values) {
      if (values.empty()) {
          kind_ = ValueKind::Nil;
      } else {
          kind_ = ValueKind::Cons;
          car_ = std::make_shared<Value>(values[0]);
          cdr_ = std::make_shared<Value>(std::vector<Value>(values.begin() + 1, values.end()));
      }
  }

  std::string to_string() const {
    switch (kind_) {
    case ValueKind::Number: {
      return std::to_string(number_);
    }
    case ValueKind::Nil: {
      return "nil";
    }
    case ValueKind::Cons: {
      if (is_list()) {
        std::string s = "(";
        for (const Value *v = this; v->kind_ != ValueKind::Nil; v = v->cdr_.get()) {
          if (v != this) {
            s += " ";
          }
          s += v->car_->to_string();
        }
        return s + ")";
      } else {
        std::string l = car_->to_string();
        std::string r = cdr_->to_string();
        return "(" + l + " . " + r + ")";
      }
    }
    }
    assert(false);
    return "";
  }

  bool is_list() const {
    switch (kind_) {
      case ValueKind::Number: return false;
      case ValueKind::Nil: return true;
      case ValueKind::Cons: return cdr_->is_list();
    }
    assert(false);
    return false;
  }

  ValueKind kind_;
  int64_t number_;
  std::shared_ptr<Value> car_;
  std::shared_ptr<Value> cdr_;
};

int msb(int64_t v) {
  int r = 0;
  while (v) {
    v >>= 1;
    r++;
  }
  return r;
}

void modulate_inner(Value *value, std::string &res) {
  const std::string POSITIVE = "01";
  const std::string NEGATIVE = "10";

  if (!value) {
    std::cerr << "modulate nullptr" << std::endl;
    exit(1);
  }
  switch (value->kind_) {
  case ValueKind::Number: {
    int64_t num = value->number_;
    if (num >= 0)
      res += POSITIVE;
    else
      res += NEGATIVE;
    num = std::abs(num);

    int t = (msb(num) + 3) / 4;
    res += std::string(t, '1');
    res += '0';

    for (int i = t * 4 - 1; i >= 0; --i)
      res += ((num >> i) & 1) ? '1' : '0';
    break;
  }
  case ValueKind::Nil: {
    res += "00";
    break;
  }
  case ValueKind::Cons: {
    res += "11";
    modulate_inner(value->car_.get(), res);
    modulate_inner(value->cdr_.get(), res);
    break;
  }
  }
}

std::string modulate(Value *value) {
  std::string res;
  modulate_inner(value, res);
  return res;
}

Value demodulate_inner(const std::string &s, int &pos) {
  if (pos >= s.length() - 1) {
    std::cerr << "demodulate: Invalid string" << std::endl;
    return Value();
  }

  if (s[pos] == '0' && s[pos + 1] == '0') {
    pos += 2;
    return Value();
  } else if (s[pos] == '1' && s[pos + 1] == '1') {
    pos += 2;
    Value car = demodulate_inner(s, pos);
    Value cdr = demodulate_inner(s, pos);
    return Value(&car, &cdr);
  } else {
    bool sign = s[pos + 1] == '1';
    pos += 2;
    int t = 0;
    for (; pos < s.length() && s[pos] == '1'; ++pos)
      t++;
    pos++;
    int64_t v = 0;
    for (int i = t * 4 - 1; i >= 0; --i) {
      v |= (s[pos++] == '1' ? 1 : 0) << i;
    }
    return Value(sign ? v : -v);
  }
}

Value demodulate(const std::string &s) {
  int pos = 0;
  return demodulate_inner(s, pos);
}
