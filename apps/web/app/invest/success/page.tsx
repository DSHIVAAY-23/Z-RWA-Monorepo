"use client";

import React, { useEffect, useState } from 'react';
import Link from 'next/link';

export default function InvestSuccessPage() {
  const [paymentId, setPaymentId] = useState<string>('');
  const [walletAddress, setWalletAddress] = useState<string>('');
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    // Extract URL params client-side
    const params = new URLSearchParams(window.location.search);
    const session = params.get('session') || params.get('paymentId') || '';
    const wallet = params.get('wallet') || '';
    setPaymentId(session);
    setWalletAddress(wallet);

    // Also try to recover wallet from paymentStore via payment-status endpoint
    if (session && !wallet) {
      fetch(`/api/payment-status/${session}`)
        .then((r) => r.json())
        .then((data) => {
          if (data?.walletAddress) setWalletAddress(data.walletAddress);
        })
        .catch(() => {});
    }
  }, []);

  const handleCopy = (text: string) => {
    navigator.clipboard.writeText(text).then(() => {
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    });
  };

  const explorerUrl = walletAddress
    ? `https://explorer.solana.com/address/${walletAddress}?cluster=devnet`
    : 'https://explorer.solana.com/?cluster=devnet';

  return (
    <div
      style={{
        minHeight: '100vh',
        background: '#000',
        color: '#fff',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        padding: '24px',
        fontFamily: "'Inter', system-ui, sans-serif",
      }}
    >
      {/* Ambient glow */}
      <div
        style={{
          position: 'fixed',
          top: '-120px',
          left: '50%',
          transform: 'translateX(-50%)',
          width: '500px',
          height: '500px',
          background: 'radial-gradient(circle, rgba(34,197,94,0.12) 0%, transparent 70%)',
          pointerEvents: 'none',
        }}
      />

      <div style={{ maxWidth: '520px', width: '100%', position: 'relative' }}>
        {/* Header nav */}
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '40px' }}>
          <span
            style={{
              background: 'linear-gradient(90deg, #a855f7, #6366f1)',
              WebkitBackgroundClip: 'text',
              WebkitTextFillColor: 'transparent',
              fontWeight: 800,
              fontSize: '20px',
              letterSpacing: '-0.5px',
            }}
          >
            Z-RWA
          </span>
          <Link
            href="/"
            style={{
              color: '#6b7280',
              textDecoration: 'none',
              fontSize: '14px',
              border: '1px solid #374151',
              borderRadius: '8px',
              padding: '6px 14px',
              transition: 'color 0.2s',
            }}
          >
            ← Back to Home
          </Link>
        </div>

        {/* Card */}
        <div
          style={{
            background: 'rgba(17,24,39,0.7)',
            backdropFilter: 'blur(20px)',
            border: '1px solid rgba(55,65,81,0.8)',
            borderRadius: '20px',
            padding: '40px 32px',
            boxShadow: '0 25px 60px rgba(0,0,0,0.5), 0 0 0 1px rgba(255,255,255,0.04)',
            position: 'relative',
            overflow: 'hidden',
          }}
        >
          {/* Green top accent */}
          <div
            style={{
              position: 'absolute',
              top: 0,
              left: 0,
              right: 0,
              height: '2px',
              background: 'linear-gradient(90deg, transparent, #22c55e, transparent)',
            }}
          />

          {/* Success Icon */}
          <div
            style={{
              width: '72px',
              height: '72px',
              borderRadius: '50%',
              background: 'rgba(34,197,94,0.15)',
              border: '2px solid rgba(34,197,94,0.4)',
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              margin: '0 auto 24px',
              fontSize: '32px',
              boxShadow: '0 0 30px rgba(34,197,94,0.25)',
            }}
          >
            ✓
          </div>

          {/* Title */}
          <h1
            style={{
              textAlign: 'center',
              fontSize: '26px',
              fontWeight: 800,
              marginBottom: '8px',
              letterSpacing: '-0.5px',
            }}
          >
            Payment Successful!
          </h1>

          {/* Subtitle */}
          <p
            style={{
              textAlign: 'center',
              color: '#a855f7',
              fontWeight: 600,
              fontSize: '15px',
              marginBottom: '28px',
              letterSpacing: '0.02em',
            }}
          >
            ZK Proof Being Generated...
          </p>

          {/* Progress steps */}
          <div
            style={{
              background: 'rgba(0,0,0,0.4)',
              border: '1px solid rgba(55,65,81,0.6)',
              borderRadius: '12px',
              padding: '20px',
              marginBottom: '24px',
            }}
          >
            {[
              { label: 'INR Payment Confirmed', done: true },
              { label: 'ZK Compliance Proof Generating', done: false, active: true },
              { label: 'Token2022 RWA Token Minting', done: false },
            ].map((step, i) => (
              <div
                key={i}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '12px',
                  padding: '8px 0',
                  borderBottom: i < 2 ? '1px solid rgba(55,65,81,0.4)' : 'none',
                }}
              >
                <div
                  style={{
                    width: '22px',
                    height: '22px',
                    borderRadius: '50%',
                    flexShrink: 0,
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    fontSize: '12px',
                    background: step.done
                      ? 'rgba(34,197,94,0.2)'
                      : step.active
                      ? 'rgba(168,85,247,0.2)'
                      : 'rgba(55,65,81,0.5)',
                    border: step.done
                      ? '1px solid rgba(34,197,94,0.5)'
                      : step.active
                      ? '1px solid rgba(168,85,247,0.5)'
                      : '1px solid rgba(75,85,99,0.4)',
                    color: step.done ? '#22c55e' : step.active ? '#a855f7' : '#6b7280',
                  }}
                >
                  {step.done ? '✓' : step.active ? '⋯' : '○'}
                </div>
                <span
                  style={{
                    fontSize: '14px',
                    color: step.done ? '#d1fae5' : step.active ? '#e9d5ff' : '#6b7280',
                    fontWeight: step.active ? 600 : 400,
                  }}
                >
                  {step.label}
                </span>
                {step.active && (
                  <span
                    style={{
                      marginLeft: 'auto',
                      fontSize: '11px',
                      color: '#a855f7',
                      fontWeight: 600,
                      padding: '2px 8px',
                      background: 'rgba(168,85,247,0.1)',
                      borderRadius: '999px',
                      border: '1px solid rgba(168,85,247,0.3)',
                      animation: 'pulse 2s infinite',
                    }}
                  >
                    IN PROGRESS
                  </span>
                )}
              </div>
            ))}
          </div>

          {/* Info message */}
          <div
            style={{
              background: 'rgba(168,85,247,0.06)',
              border: '1px solid rgba(168,85,247,0.2)',
              borderRadius: '10px',
              padding: '14px 16px',
              marginBottom: '24px',
              fontSize: '13px',
              color: '#c4b5fd',
              lineHeight: '1.6',
            }}
          >
            Your Token2022 RWA token will be minted to your wallet once the ZK proof is verified on Solana devnet. This usually completes in under 60 seconds.
          </div>

          {/* Payment ID */}
          {paymentId && (
            <div style={{ marginBottom: '16px' }}>
              <div style={{ fontSize: '11px', color: '#6b7280', fontFamily: 'monospace', marginBottom: '6px', letterSpacing: '0.08em' }}>
                PAYMENT ID
              </div>
              <div
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '8px',
                  background: 'rgba(0,0,0,0.5)',
                  border: '1px solid #374151',
                  borderRadius: '8px',
                  padding: '10px 14px',
                }}
              >
                <span style={{ fontFamily: 'monospace', fontSize: '12px', color: '#9ca3af', flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                  {paymentId}
                </span>
                <button
                  onClick={() => handleCopy(paymentId)}
                  style={{
                    background: 'none',
                    border: 'none',
                    cursor: 'pointer',
                    color: copied ? '#22c55e' : '#6b7280',
                    fontSize: '12px',
                    flexShrink: 0,
                    transition: 'color 0.2s',
                  }}
                >
                  {copied ? 'Copied!' : 'Copy'}
                </button>
              </div>
            </div>
          )}

          {/* Wallet address */}
          {walletAddress && (
            <div style={{ marginBottom: '28px' }}>
              <div style={{ fontSize: '11px', color: '#6b7280', fontFamily: 'monospace', marginBottom: '6px', letterSpacing: '0.08em' }}>
                DESTINATION WALLET
              </div>
              <div
                style={{
                  background: 'rgba(0,0,0,0.5)',
                  border: '1px solid #374151',
                  borderRadius: '8px',
                  padding: '10px 14px',
                  fontFamily: 'monospace',
                  fontSize: '12px',
                  color: '#9ca3af',
                  wordBreak: 'break-all',
                }}
              >
                {walletAddress}
              </div>
            </div>
          )}

          {/* CTA buttons */}
          <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
            <a
              href={explorerUrl}
              target="_blank"
              rel="noopener noreferrer"
              style={{
                display: 'block',
                textAlign: 'center',
                background: 'linear-gradient(135deg, #7c3aed, #4f46e5)',
                color: '#fff',
                textDecoration: 'none',
                fontWeight: 700,
                fontSize: '15px',
                padding: '14px',
                borderRadius: '10px',
                boxShadow: '0 0 20px rgba(124,58,237,0.3)',
                transition: 'opacity 0.2s',
              }}
            >
              View on Solana Explorer ↗
            </a>

            <Link
              href="/"
              style={{
                display: 'block',
                textAlign: 'center',
                background: 'transparent',
                color: '#9ca3af',
                textDecoration: 'none',
                fontWeight: 500,
                fontSize: '14px',
                padding: '12px',
                borderRadius: '10px',
                border: '1px solid #374151',
                transition: 'color 0.2s, border-color 0.2s',
              }}
            >
              ← Back to Home
            </Link>
          </div>
        </div>

        {/* Footer note */}
        <p
          style={{
            textAlign: 'center',
            marginTop: '24px',
            fontSize: '12px',
            color: '#4b5563',
          }}
        >
          Powered by Z-RWA · SP1 ZK Proofs · Token2022 · Solana Devnet
        </p>
      </div>

      <style>{`
        @keyframes pulse {
          0%, 100% { opacity: 1; }
          50% { opacity: 0.5; }
        }
      `}</style>
    </div>
  );
}
