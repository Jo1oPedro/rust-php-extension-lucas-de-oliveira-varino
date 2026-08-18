<?php

// Stubs for extensao_varinha

namespace Varinha {
    class Vector {
        /**
         * @param int $size
         * @param string $vector_type
         */
        public function __construct(int $size, string $vector_type) {}

        /**
         * @return int
         */
        public function size(): int {}

        /**
         * @return string
         */
        public function teste(): string {}

        /**
         * @return string
         */
        public function vectorType(): string {}
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
