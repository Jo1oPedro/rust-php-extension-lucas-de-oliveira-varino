<?php

// Stubs for extensao_varinha

namespace Varinha {
    class VarinhaArray {
        /**
         * @param int $capacity
         * @param string $array_type
         */
        public function __construct(int $capacity, string $array_type) {}

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

    class VarinhaQueue {
        /**
         * @param array|null $initial_values
         */
        public function __construct(?array $initial_values = null) {}

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
    }

    class VarinhaVector {
        public function __construct() {}

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

namespace {
    /**
     * @return string
     */
    function call_varinha(): string {}

    /**
     * @return array
     */
    function call_varinha_headers(): array {}

    /**
     * @param string $mensagem
     * @return void
     */
    function varinha(string $mensagem): void {}
}
