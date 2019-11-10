#include "backend.hpp"
#include <iostream>

Backend::Backend(QObject *parent, Callbacks callbacks) : QObject(parent)
{
    _callbacks = callbacks;

    _callbacks.init(callbacks.data, this);
}

Backend::~Backend()
{
    _callbacks.deleteData(_callbacks.data);
}

void Backend::updatedTextField(QString str)
{
    _callbacks.updatedTextField(_callbacks.data, str.toUtf8().data());
}
