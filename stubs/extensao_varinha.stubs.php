<?php

// Stubs for extensao_varinha

namespace Varinha {
    class Array {
        /**
         * @param int $capacity
         * @param string $array_type
         */
        public function __construct(int $capacity, string $array_type) {}

        /**
         * @param mixed $valor
         * @return void
         */
        public function adicionar(mixed $valor): void {}

        /**
         * @return string
         */
        public function arrayType(): string {}

        /**
         * @return array
         */
        public function getAll(): array {}

        /**
         * @return int
         */
        public function size(): int {}

        /**
         * @return string
         */
        public function teste(): string {}
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
