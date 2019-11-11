#pragma once

extern "C" {
    struct Backend;

    struct Callbacks {
        void (*init)(void* data, Backend* internal);
        void (*updatedTextField)(void* data, char* text);
        void (*solve)(void* data);
        void (*deleteData)(void* data);
        void* data;
    };

    int runGui(Callbacks callbacks);
    void setAnswer(Backend* internal, char* value);
    void invalidInput(Backend* internal, char* fixed);
}
