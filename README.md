# rust-eratos
An implementation of 'Sieve of Eratosthenes' for rust language practice.

## Usage
```
cargo install rust-eratos
rust-eratos 11
```

- - -

# Rust 언어에 적응하기

## 이 문서의 목표

- Rust 언어에 대한 기본적인 이해를 마친 초심자에게
- 언어를 직접 사용하며 몸에 익히고 적응하는데 도움을 주고자
- 가볍게 따라할 수 있는 작은 목표들을 단계적으로 제공한다.

## 사전 준비

- Rust 언어의 기본 문법을 대략적으로 알고 있거나 쉽게 찾아볼 수 있어야 한다.
    - 혹시나 처음이라면 취향에 맞는 것으로 준비한다.
        - 짧지만 맛보기는 가능한 마소 런 튜토리얼
        [Rust First Steps](https://docs.microsoft.com/ko-kr/learn/paths/rust-first-steps/)
        - 길지만 훌륭한 공식 커뮤니티 튜토리얼
        [The Rust Programming Language](https://doc.rust-lang.org/book/)
        - 온라인 강의
        [Exercism: Rust Track](https://exercism.org/tracks/rust)
        - 유툽 강의
        [YouTube: mithradates](https://www.youtube.com/user/mithradates/videos)
- 소수(prime number)를 판별하는 간단한 알고리즘 지식이 필요하다.
- 미리 학습할 필요는 없고, 따라하면서 필요한 내용을 찾아본다.
[소수 (수론)](https://ko.wikipedia.org/wiki/%EC%86%8C%EC%88%98_(%EC%88%98%EB%A1%A0))

## 지금 단계에서 집중해야 할 것

- 기본 문법을 완전히 습득하는 것
    - primitive types, variable, if, match, for, println!, fn, ...
- 기본 자료구조들에 익숙해지는 것
    - array, slice, String, Vec, HashMap, tuple, ...
- 언어 공통적 요소를 사용해보는 것
    - struct, enum, trait, iterator, ...
- 언어 특수한 요소를 경험하고 이해하는 것
    - ownership, borrow, reference, macros, ...
- 언어 생태계를 경험하는 것
    - cargo, crates, ...
- 이번 따라하기에서 위의 것들을 전부 사용해보는 것은 아님

## 지금은 집중하지 않을 것

- 알고리즘의 개선
- 특정 프레임워크의 사용 방법
- 언어와 생태계의 고급 기능들
    - generic, polymorphism, thread, future, async, cargo workspace, ...

## 따라하기

- 제시된 함수의 내용을 구현한다.
- 만약 알고리즘을 잘 이해하지 못하겠고 생각하기도 귀찮다면, 문서 하단의 예시 코드를 적절하게 포팅하는 방식으로 따라한다.
- 제공된 인터페이스들은 입력 받은 수 n을 포함하지 않는 결과를 리턴하도록 의도하였으나, 포함시켜도 무관하다. 다만 일관된 동작으로 작성한다.

### *n* 이 소수인지 판별하기

```rust
fn is_prime_number(n: u32) -> bool
```

- 혹시 if, for 명령들로만 구현했다면 match, range, iter, any 등을 이용한 표현으로 구현해본다.

### *n* 미만의 소수의 수를 구하기

```rust
fn get_prime_number_count_below(n: u32) -> usize
```

- 혹시 if, for 명령들로만 구현했다면 match, range, iter, filter, count 등을 이용한 표현으로 구현해본다.
- filter를 사용했다면 앞에서 사용했던 any의 것과 비교하여 타입의 차이와 그 이유를 알아본다.
    - hint: Iterator::next(&mut self) vs new iterator

### *n* 미만의 가장 큰 소수를 구하기

```rust
fn get_largest_prime_number_below(n: u32) -> u32
```

- 혹시 if, for 명령들로만 구현했다면 match, range, iter, find, Option&lt;T&gt; 등을 이용한 표현으로 구현해본다.
- find를 사용했다면 앞에서 사용했던 any, filter들과 비교하여 타입의 차이와 그 이유를 알아본다.

### *n* 미만의 소수들을 구하기

```rust
fn get_prime_numbers_below(n: u32) -> Vec<u32>
```

- 에라토스테네스의 체 알고리즘을 구현한다.
- 기본 자료구조 중 Vec을 사용하여 구현해본다.
- 혹시 if, for 명령들로만 구현했다면 range, into_iter, filter, collect 등을 이용한 표현으로 구현해본다.

### 커맨드 라인 입력을 정수로 변환하기
```rust
fn parse_args(args: &[String]) -> Result<u32, &'static str>
```

- 커맨드 라인으로 입력받은 문자열을 정수로 변환한다.
- 정수 변환 결과와 오류를 Result&lt;T, E&gt; 를 이용하여 적절하게 반환하고 처리해본다.

### 작성한 기능들로 stdout에 출력하기

```sh
# output example
> rust-eratos 13

13 is a prime number.
There are 5 prime numbers less than 13, and the largest number is 11.
Prime numbers less than 13.
[2, 3, 5, 7, 11]
```

### 더 해보기
- 유닛 테스트 작성하기
- 구현된 기능들을 라이브러리로 만들기
- 계산된 결과를 캐싱하여 성능 개선하기
- 코드의 실행 속도를 프로파일하고 개선하기
- {2, 3, 5} 휠 인수분해 알고리즘 적용하기
    - Wheel factorization
    [Link](https://en.wikipedia.org/wiki/Wheel_factorization),
    [more](https://primes.utm.edu/glossary/page.php?sort=WheelFactorization)

## 예시 코드

```python
# python

import sys


def is_prime_number(n):
    if n < 2:
        return False

    for i in range(2, int(n ** 0.5) + 1):  # (n ** 0.5) == (math.sqrt(n))
        if n % i == 0:
            return False

    return True


def get_prime_number_count_below(n):
    if n < 3:
        return 0

    count = 0
    for i in range(2, n):
        if is_prime_number(i):
            count += 1

    return count


def get_largest_prime_number_below(n):
    for i in reversed(range(2, n)):
        if is_prime_number(i):
            return i

    return 0


def get_prime_numbers_below(n):
    # sieve = [0, 0] + [i for i in range(2, n)]
    sieve = [0, 0]
    for i in range(2, n):
        sieve.append(i)

    for i in range(2, int(len(sieve) ** 0.5) + 1):
        if sieve[i] <= 0:
            continue

        index = i * i
        while index < len(sieve):
            sieve[index] = 0
            index += i

    primes = []
    for i in range(2, len(sieve)):
        if sieve[i] > 0:
            primes.append(i)

    return primes
    # return [i for i in range(2, len(sieve)) if sieve[i] > 0]


def main(n):
    print(f"{n} is a prime number.")
    print(f"There are {get_prime_number_count_below(n)} prime numbers less than {n},"
          f" and the largest number is {get_largest_prime_number_below(n)}")
    print(f"Prime numbers less than {n}")
    print(get_prime_numbers_below(n))


if __name__ == "__main__":
    main(int(sys.argv[1]))

```
