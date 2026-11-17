#pragma once

#ifndef PALIKEY_TSF_CLASS_FACTORY_H
#define PALIKEY_TSF_CLASS_FACTORY_H

#include <windows.h>
#include <unknwn.h>

class CPaliKeyClassFactory : public IClassFactory {
public:
    CPaliKeyClassFactory();
    virtual ~CPaliKeyClassFactory();

    // IUnknown
    STDMETHODIMP QueryInterface(REFIID riid, void **ppvObj) override;
    STDMETHODIMP_(ULONG) AddRef() override;
    STDMETHODIMP_(ULONG) Release() override;

    // IClassFactory
    STDMETHODIMP CreateInstance(IUnknown *pUnkOuter, REFIID riid, void **ppvObject) override;
    STDMETHODIMP LockServer(BOOL fLock) override;

private:
    LONG m_cRef;
};

#endif // PALIKEY_TSF_CLASS_FACTORY_H
