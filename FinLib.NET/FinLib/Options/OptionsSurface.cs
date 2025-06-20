using System;

namespace FinLib.Options;

public class OptionsSurface: IDisposable
{
    private readonly unsafe OptionsSurface_native* _handle;
    internal unsafe OptionsSurface_native* GetPtr() => _handle;

    internal unsafe OptionsSurface(OptionsSurface_native* handle)
    {
        _handle = handle;
    }

    public void Generate()
    {
        unsafe
        {
            NativeMethods.option_surface_generate(_handle);
        }
    }

    public void ParGenerate()
    {
        unsafe
        {
            NativeMethods.option_surface_par_generate(_handle);
        }
    }

    private void ReleaseUnmanagedResources()
    {
        unsafe
        {
            NativeMethods.option_surface_destroy(_handle);
        }
    }

    public void Dispose()
    {
        ReleaseUnmanagedResources();
        GC.SuppressFinalize(this);
    }

    ~OptionsSurface()
    {
        ReleaseUnmanagedResources();
    }
}