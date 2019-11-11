#include "backend.hpp"
#include <iostream>

Backend::Backend(QObject *parent) : QObject(parent)
{
    _callbacks = emptyCallbacks();
}

void Backend::setCallbacks(Callbacks callbacks)
{
    _callbacks = callbacks;

    _callbacks.init(_callbacks.data, this);
}

Backend::~Backend()
{
    _callbacks.deleteData(_callbacks.data);
}

void Backend::updatedTextField(QString str)
{
    _callbacks.updatedTextField(_callbacks.data, str.toUtf8().data());
}

Callbacks Backend::emptyCallbacks()
{
    return Callbacks {
        nullptr, nullptr, nullptr, nullptr, nullptr
    };
}
