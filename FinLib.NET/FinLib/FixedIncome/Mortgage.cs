using System;

namespace FinLib.FixedIncome;

public class Mortgage: IDisposable
{
    private readonly unsafe Mortgage_native* _handle;
    internal unsafe Mortgage_native* GetPtr() => _handle;

    public Mortgage(double purchasePrice, double deposit, double interestRate, int termYears)
    {
        unsafe
        {
            _handle = NativeMethods.mortgage_new(purchasePrice, deposit, interestRate, termYears);
        }
    }

    public double LTV
    {
        get
        {
            unsafe
            {
                return NativeMethods.mortgage_ltv(_handle);
            }
        }
    }

    public double LoanValue
    {
        get
        {
            unsafe
            {
                return NativeMethods.mortgage_loan_value(_handle);
            }
        }
    }

    public double MonthlyPayment
    {
        get
        {
            unsafe
            {
                return NativeMethods.mortgage_monthly_payment(_handle);
            }
        }
    }

    public double TotalRepayment
    {
        get
        {
            unsafe
            {
                return NativeMethods.mortgage_total_repayment(_handle);
            }
        }
    }

    public double TotalInterestRepayment
    {
        get
        {
            unsafe
            {
                return NativeMethods.mortgage_total_interest_repayment(_handle);
            }
        }
    }

    public double PresentValue
    {
        get
        {
            unsafe
            {
                return NativeMethods.mortgage_present_value(_handle);
            }
        }
    }

    public double FutureValue(double annualInterestRate)
    {
        unsafe
        {
            return NativeMethods.mortgage_future_value(_handle, annualInterestRate);
        }
    }

    public double NetFutureValueInterest(double annualInterestRate)
    {
        unsafe
        {
            return NativeMethods.mortgage_net_future_value_interest(_handle, annualInterestRate);
        }
    }

    public double TotalInterest(double annualInterestRate)
    {
        unsafe
        {
            return NativeMethods.mortgage_total_interest(_handle, annualInterestRate);
        }
    }

    private void ReleaseUnmanagedResources()
    {
        unsafe
        {
            NativeMethods.mortgage_destroy(_handle);
        }
    }

    public void Dispose()
    {
        ReleaseUnmanagedResources();
        GC.SuppressFinalize(this);
    }

    ~Mortgage()
    {
        ReleaseUnmanagedResources();
    }
}