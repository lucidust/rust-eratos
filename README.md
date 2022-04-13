# rust-eratos
An implementation of 'Sieve of Eratosthenes' for rust language practice.

## Usage
```
cargo install rust-eratos
rust-eratos 11
```

- - -

# 새로운 언어에 적응하기

## 이 문서의 목표

- 새로 배우려는 언어에 대해 기본적인 이해를 마친 초심자에게
- 언어를 직접 사용하며 몸에 익히고 적응하는데 도움을 주고자
- 가볍게 따라할 수 있는 작은 목표들을 단계적으로 제공한다.

## 사전 지식

- 소수(prime number)를 판별하는 간단한 알고리즘 지식이 필요하다.
- 미리 학습할 필요는 없고, 따라하면서 필요한 내용을 찾아본다.
- Link: [소수 (수론)](https://ko.wikipedia.org/wiki/%EC%86%8C%EC%88%98_(%EC%88%98%EB%A1%A0))

## 집중해야 할 것

> 언어의 사용에 익숙해지는 것에 집중한다.

- 기본 문법을 완전히 습득하는 것
    - 기본 타입과 변수, 분기, 반복, 콘솔 출력, 함수, ...
- 기본 자료구조들에 익숙해지는 것
    - string, array, slice, list, hashmap, tuple, ...
- 언어 공통적 요소를 사용해보는 것
    - struct, class, enum, traits, ...
- 언어 특수한 요소를 경험하고 이해하는 것
    - ownership, borrow, reference, ...
- 언어 생태계를 경험하는 것
    - nuget, pip, cargo, npm, go package, ...

## 집중하지 말아야 할 것

- 알고리즘의 개선
- 특정 프레임워크의 사용 방법
- 언어의 고급 기능들
    - generic, polymorphism, thread, future, async, channel, go routine, ...

## 따라하기

- 만약 알고리즘을 잘 이해하지 못하겠고 생각하기도 귀찮다면, 문서 하단의 예시 코드를 적절하게 포팅하는 방식으로 따라한다.
- 제공된 인터페이스들은 입력 받은 수 n을 포함하지 않는 결과를 리턴하도록 의도하였으나, 포함시켜도 무관하다. 다만 일관된 동작으로 작성한다.

### *n* 이 소수인지 판별하기

> 2, 3, 5, 7, 11, 13 등과 같이 1보다 큰 자연수 중에서 1과 자기 자신만을 약수로 가지는 수를 소수라고 한다.

```rust
is_prime_number(n: u32) -> bool
```

- 분기, 반복, 함수 등 기본 표현법을 사용해볼 수 있다.
- Rust
    - 분기와 반복 명령 등을 이용하는 대신 range, iter 등을 이용한 표현법으로 구현을 해볼 수 있다.

### *n* 미만의 소수의 수를 구하기

```rust
get_prime_number_count_below(n: u32) -> usize
```

- 자료구조 중 list를 사용해볼 수 있다.
- Rust
    - list에 대응되는 기본 자료구조인 Vec을 사용해볼 수 있다.
    - 분기와 반복 명령 등을 이용하는 대신 range, iter, filter 등을 이용한 표현법으로 구현을 해볼 수 있다.

### *n* 미만의 가장 큰 소수를 구하기

```rust
get_largest_prime_number_below(n: u32) -> u32
```

- Rust
    - if 분기 대신 match, Option&lt;T&gt;를 이용하여 분기를 표현해볼 수 있다.

### *n* 미만의 소수들을 구하기

> 에라토스테네스의 체 알고리즘을 구현한다.

```rust
get_prime_numbers_below(n: u32) -> Vec<u32>
```

- 자료구조 중 list를 사용해볼 수 있다.
- Rust
    - list에 대응되는 기본 자료구조인 Vec을 사용해볼 수 있다.
    - 분기와 반복 명령 등을 이용하는 대신 range, iter, filter 등을 이용한 표현법으로 구현을 해볼 수 있다.
    - mut 키워드를 사용해볼 수 있다.
    - Vec의 값들에 접근하고 수정하면서 reference, mutable reference에 대해 생각해볼 수 있다.
    - 소유권에 대해 생각해볼 수 있다.

    - for 문을 사용했나요? range, iter를 사용하는 표현으로 변경해보세요.

### 작성한 기능들로 stdout에 출력하기

```
# output example
> rust-eratos 13

13 is a prime number.
There are 5 prime numbers less than 13, and the largest number is 11.
Prime numbers less than 13.
[ 2  ], [ 3  ], [ 5  ], [ 7  ], [ 11 ],
```

### *n*을 커맨드 라인 입력으로 받아 처리하기

### 에러 처리하기

stderr로 에러 출력하기

### 구현된 기능들을 라이브러리로 만들기

### 계산된 결과를 캐싱하여 성능 개선하기

### 유닛 테스트 작성하기

### 코드의 실행 속도를 프로파일하고 개선하기

### {2, 3, 5} 휠 인수분해 알고리즘 적용하기

Wheel factorization
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
