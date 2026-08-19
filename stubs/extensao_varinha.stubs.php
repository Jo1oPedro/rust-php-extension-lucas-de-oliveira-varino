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
         * @param mixed $valor
         * @return int
         */
        public function add(mixed $valor): int {}

        /**
         * @return string
         */
        public function arrayType(): string {}

        /**
         * @return int
         */
        public function capacity(): int {}

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
         * @param int $index
         * @param mixed $valor
         * @return int
         */
        public function set(int $index, mixed $valor): int {}

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
