<?php

// Stubs for extensao_ds

namespace Ds {
    class DSArray {
        /**
         * @param int $capacity
         * @param string $array_type
         */
        public function __construct(int $capacity, string $array_type) {}

        /**
         * @return array
         */
        public function __debugInfo(): array {}

        /**
         * @param mixed $value
         * @return int
         */
        public function add(mixed $value): int {}

        /**
         * @return string
         */
        public function arrayType(): string {}

        /**
         * @return int
         */
        public function capacity(): int {}

        /**
         * @return void
         */
        public function clean(): void {}

        /**
         * @return mixed
         */
        public function first(): mixed {}

        /**
         * @param int $index
         * @return mixed
         */
        public function get(int $index): mixed {}

        /**
         * @return array
         */
        public function getAll(): array {}

        /**
         * @param mixed $needle
         * @return int
         */
        public function indexOf(mixed $needle): int {}

        /**
         * @param int $index
         * @param mixed $value
         * @return int
         */
        public function insert(int $index, mixed $value): int {}

        /**
         * @return bool
         */
        public function isEmpty(): bool {}

        /**
         * @return bool
         */
        public function isFull(): bool {}

        /**
         * @return mixed
         */
        public function last(): mixed {}

        /**
         * @return mixed
         */
        public function pop(): mixed {}

        /**
         * @param int $index
         * @return mixed
         */
        public function remove(int $index): mixed {}

        /**
         * @param mixed $needle
         * @return mixed
         */
        public function search(mixed $needle): mixed {}

        /**
         * @param int $index
         * @param mixed $value
         * @return int
         */
        public function set(int $index, mixed $value): int {}

        /**
         * @return int
         */
        public function size(): int {}
    }

    class DSBTreeMap {
        /**
         * @param array|null $initial_values
         */
        public function __construct(?array $initial_values = null) {}

        /**
         * @return array
         */
        public function __debugInfo(): array {}

        /**
         * @return void
         */
        public function clean(): void {}

        /**
         * @return mixed
         */
        public function first(): mixed {}

        /**
         * @return string
         */
        public function firstKey(): string {}

        /**
         * @param string $key
         * @return mixed
         */
        public function get(string $key): mixed {}

        /**
         * @param string $key
         * @return bool
         */
        public function has(string $key): bool {}

        /**
         * @return bool
         */
        public function isEmpty(): bool {}

        /**
         * @return mixed
         */
        public function last(): mixed {}

        /**
         * @return string
         */
        public function lastKey(): string {}

        /**
         * @param string $key
         * @param mixed $value
         * @return void
         */
        public function put(string $key, mixed $value): void {}

        /**
         * @param string|null $from
         * @param string|null $to
         * @return array
         */
        public function range(?string $from = null, ?string $to = null): array {}

        /**
         * @param string $key
         * @return mixed
         */
        public function remove(string $key): mixed {}

        /**
         * @return int
         */
        public function size(): int {}

        /**
         * @return array
         */
        public function toArray(): array {}
    }

    class DSHashMap {
        /**
         * @param array|null $initial_values
         */
        public function __construct(?array $initial_values = null) {}

        /**
         * @return array
         */
        public function __debugInfo(): array {}

        /**
         * @return void
         */
        public function clean(): void {}

        /**
         * @param string $key
         * @return mixed
         */
        public function get(string $key): mixed {}

        /**
         * @param string $key
         * @return bool
         */
        public function has(string $key): bool {}

        /**
         * @return bool
         */
        public function isEmpty(): bool {}

        /**
         * @return array
         */
        public function keys(): array {}

        /**
         * @param string $key
         * @param mixed $value
         * @return int
         */
        public function put(string $key, mixed $value): int {}

        /**
         * @param string $key
         * @return mixed
         */
        public function remove(string $key): mixed {}

        /**
         * @return int
         */
        public function size(): int {}

        /**
         * @return array
         */
        public function toArray(): array {}

        /**
         * @return array
         */
        public function values(): array {}
    }

    class DSHashSet {
        /**
         * @param array|null $initial_values
         */
        public function __construct(?array $initial_values = null) {}

        /**
         * @return array
         */
        public function __debugInfo(): array {}

        /**
         * @param string $value
         * @return bool
         */
        public function add(string $value): bool {}

        /**
         * @return void
         */
        public function clean(): void {}

        /**
         * @param string $value
         * @return bool
         */
        public function contains(string $value): bool {}

        /**
         * @param \Ds\DSHashSet $other
         * @return \Ds\DSHashSet
         */
        public function difference(\Ds\DSHashSet $other): \Ds\DSHashSet {}

        /**
         * @param \Ds\DSHashSet $other
         * @return \Ds\DSHashSet
         */
        public function intersection(\Ds\DSHashSet $other): \Ds\DSHashSet {}

        /**
         * @return bool
         */
        public function isEmpty(): bool {}

        /**
         * @param \Ds\DSHashSet $other
         * @return bool
         */
        public function isSubset(\Ds\DSHashSet $other): bool {}

        /**
         * @param string $value
         * @return bool
         */
        public function remove(string $value): bool {}

        /**
         * @return int
         */
        public function size(): int {}

        /**
         * @return array
         */
        public function toArray(): array {}

        /**
         * @param \Ds\DSHashSet $other
         * @return \Ds\DSHashSet
         */
        public function union(\Ds\DSHashSet $other): \Ds\DSHashSet {}
    }

    class DSPriorityQueue {
        const MAX = false;

        const MIN = true;

        /**
         * @param array|null $intial_values
         * @param bool|null $is_min
         */
        public function __construct(?array $intial_values = null, ?bool $is_min = null) {}

        /**
         * @return array
         */
        public function __debugInfo(): array {}

        /**
         * @return void
         */
        public function clean(): void {}

        /**
         * @return bool
         */
        public function isEmpty(): bool {}

        /**
         * @return mixed
         */
        public function peek(): mixed {}

        /**
         * @param mixed $value
         * @return int
         */
        public function push(mixed $value): int {}

        /**
         * @return int
         */
        public function size(): int {}
    }

    class DSQueue {
        /**
         * @param array|null $initial_values
         */
        public function __construct(?array $initial_values = null) {}

        /**
         * @return array
         */
        public function __debugInfo(): array {}

        /**
         * @return void
         */
        public function clean(): void {}

        /**
         * @return mixed
         */
        public function dequeue(): mixed {}

        /**
         * @param mixed $value
         * @return int
         */
        public function enqueue(mixed $value): int {}

        /**
         * @return bool
         */
        public function isEmpty(): bool {}

        /**
         * @return mixed
         */
        public function peek(): mixed {}

        /**
         * @return int
         */
        public function size(): int {}

        /**
         * @return array
         */
        public function toArray(): array {}
    }

    class DSVector {
        /**
         * @param array|null $initial_values
         */
        public function __construct(?array $initial_values = null) {}

        /**
         * @return array
         */
        public function __debugInfo(): array {}

        /**
         * @param mixed $value
         * @return int
         */
        public function add(mixed $value): int {}

        /**
         * @return void
         */
        public function clean(): void {}

        /**
         * @param mixed $value
         * @return bool
         */
        public function contains(mixed $value): bool {}

        /**
         * @param array $values
         * @return int
         */
        public function extend(array $values): int {}

        /**
         * @return mixed
         */
        public function first(): mixed {}

        /**
         * @param int $index
         * @return mixed
         */
        public function get(int $index): mixed {}

        /**
         * @return array
         */
        public function getAll(): array {}

        /**
         * @param mixed $value
         * @return int
         */
        public function indexOf(mixed $value): int {}

        /**
         * @return bool
         */
        public function isEmpty(): bool {}

        /**
         * @return mixed
         */
        public function last(): mixed {}

        /**
         * @return mixed
         */
        public function pop(): mixed {}

        /**
         * @param int $index
         * @return mixed
         */
        public function remove(int $index): mixed {}

        /**
         * @param int $additional
         * @return void
         */
        public function reserve(int $additional): void {}

        /**
         * @param mixed $value
         * @return mixed
         */
        public function search(mixed $value): mixed {}

        /**
         * @param int $index
         * @param mixed $value
         * @return int
         */
        public function set(int $index, mixed $value): int {}

        /**
         * @return void
         */
        public function shrinkToFit(): void {}

        /**
         * @return int
         */
        public function size(): int {}
    }
}

namespace Ds {
    interface Prioritizable {
        /**
         * @return float
         */
        public function getPriority(): float;
    }
}
