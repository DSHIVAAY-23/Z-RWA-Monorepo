import Link from 'next/link';
import ZNavbar from '../../components/ZNavbar';

export default function PrivacyPage() {
  return (
    <div className="min-h-screen bg-[var(--background)] grid-bg transition-colors duration-200 font-sans text-[var(--foreground)]">
      <ZNavbar />

      <main className="mx-auto max-w-6xl px-6 py-12 space-y-16">
        {/* Section 1: Hero */}
        <section className="text-center space-y-6 pt-10">
          <h1 className="text-5xl font-bold font-space bg-gradient-to-br from-white to-gray-400 bg-clip-text text-transparent">
            Full-Stack Privacy for RWA
          </h1>
          <p className="text-xl text-gray-400 max-w-2xl mx-auto leading-relaxed">
            ZK proofs for identity. MagicBlock for payments. Nothing leaks on-chain.
          </p>
          <div className="flex justify-center gap-4 pt-4">
            <span className="px-5 py-2 rounded-full border border-purple-500/50 bg-purple-500/10 text-purple-400 font-mono text-sm tracking-wide shadow-[0_0_15px_rgba(168,85,247,0.2)]">
              SnarkJS Groth16
            </span>
            <span className="px-5 py-2 rounded-full border border-teal-500/50 bg-teal-500/10 text-teal-400 font-mono text-sm tracking-wide shadow-[0_0_15px_rgba(20,184,166,0.2)]">
              MagicBlock Private Payments
            </span>
          </div>
        </section>

        {/* Section 2: Privacy Model */}
        <section className="grid md:grid-cols-3 gap-6">
          <div className="p-8 rounded-2xl border border-gray-800 bg-gray-900/50 backdrop-blur-sm transition-all hover:bg-gray-900 shadow-xl">
            <div className="w-12 h-12 rounded-xl bg-purple-500/20 text-purple-400 flex items-center justify-center text-2xl mb-6">
              🛡️
            </div>
            <h3 className="text-xl font-space font-bold text-white mb-3 tracking-wide">
              Who you are stays private
            </h3>
            <p className="text-gray-400 text-sm leading-relaxed">
              Aadhaar/PAN hashed via Poseidon locally. Circom circuit generates Groth16 proof via SnarkJS. Chain sees only the proof — never your identity.
            </p>
          </div>

          <div className="p-8 rounded-2xl border border-teal-800/50 bg-teal-900/10 backdrop-blur-sm transition-all hover:bg-teal-900/20 shadow-xl">
            <div className="w-12 h-12 rounded-xl bg-teal-500/20 text-teal-400 flex items-center justify-center text-2xl mb-6">
              👁️‍🗨️
            </div>
            <h3 className="text-xl font-space font-bold text-white mb-3 tracking-wide">
              What you pay stays private
            </h3>
            <p className="text-gray-400 text-sm leading-relaxed">
              RWA purchases settled via MagicBlock Private Payments API. Amount and recipient are shielded from on-chain observers.
            </p>
          </div>

          <div className="p-8 rounded-2xl border border-gray-800 bg-gray-900/50 backdrop-blur-sm transition-all hover:bg-gray-900 shadow-xl">
            <div className="w-12 h-12 rounded-xl bg-green-500/20 text-green-400 flex items-center justify-center text-2xl mb-6">
              ✅
            </div>
            <h3 className="text-xl font-space font-bold text-white mb-3 tracking-wide">
              Regulators can still verify
            </h3>
            <p className="text-gray-400 text-sm leading-relaxed">
              ZK proofs are cryptographically verifiable. SEBI/RBI can confirm compliance without accessing personal data.
            </p>
          </div>
        </section>

        {/* Section 3: Flow diagram */}
        <section className="rounded-2xl border border-gray-800 bg-black/40 p-10 overflow-hidden">
          <h2 className="text-2xl font-space font-bold text-center mb-10 text-white">Cryptographic Data Flow</h2>
          
          <div className="flex flex-col md:flex-row items-center justify-between gap-4 max-w-5xl mx-auto">
            {/* Step 1 */}
            <div className="flex flex-col items-center w-full md:w-1/5">
              <div className="p-5 rounded-xl border border-gray-700 bg-gray-900 w-full text-center shadow-lg relative z-10 shrink-0 h-[140px] flex flex-col justify-center">
                <div className="text-white font-space font-semibold mb-2">User Device</div>
                <div className="text-xs text-gray-400 font-mono">Aadhaar/PAN<br/>(never leaves)</div>
              </div>
            </div>

            <div className="hidden md:flex text-teal-500 text-2xl mx-1">→</div>
            <div className="md:hidden text-teal-500 text-2xl my-2">↓</div>

            {/* Step 2 */}
            <div className="flex flex-col items-center w-full md:w-1/5">
              <div className="p-5 rounded-xl border border-purple-500/50 bg-purple-900/20 w-full text-center shadow-lg relative z-10 shrink-0 h-[140px] flex flex-col justify-center">
                <div className="text-purple-400 font-space font-semibold mb-2">Circom Circuit</div>
                <div className="text-xs text-purple-300 font-mono">Local computation<br/>Groth16 proof out</div>
              </div>
            </div>

            <div className="hidden md:flex text-teal-500 text-2xl mx-1">→</div>
            <div className="md:hidden text-teal-500 text-2xl my-2">↓</div>

            {/* Step 3 */}
            <div className="flex flex-col items-center w-full md:w-1/5">
              <div className="p-5 rounded-xl border border-gray-700 bg-gray-900 w-full text-center shadow-lg relative z-10 shrink-0 h-[140px] flex flex-col justify-center">
                <div className="text-white font-space font-semibold mb-2">Solana Anchor</div>
                <div className="text-xs text-gray-400 font-mono">Proof verified<br/>Token2022 gated</div>
              </div>
            </div>

            <div className="hidden md:flex text-teal-500 text-2xl mx-1">→</div>
            <div className="md:hidden text-teal-500 text-2xl my-2">↓</div>

            {/* Step 4 */}
            <div className="flex flex-col items-center w-full md:w-1/5">
              <div className="p-5 rounded-xl border border-teal-500/50 bg-teal-900/20 w-full text-center shadow-lg relative z-10 shrink-0 h-[140px] flex flex-col justify-center">
                <div className="text-teal-400 font-space font-semibold mb-2">MagicBlock PER</div>
                <div className="text-xs text-teal-300 font-mono">Private payment<br/>Amount shielded</div>
              </div>
            </div>

            <div className="hidden md:flex text-teal-500 text-2xl mx-1">→</div>
            <div className="md:hidden text-teal-500 text-2xl my-2">↓</div>

            {/* Step 5 */}
            <div className="flex flex-col items-center w-full md:w-1/5">
              <div className="p-5 rounded-xl border border-green-500/50 bg-green-900/20 w-full text-center shadow-lg relative z-10 shrink-0 h-[140px] flex flex-col justify-center">
                <div className="text-green-400 font-space font-semibold mb-2">RWA Token</div>
                <div className="text-xs text-green-300 font-mono">Delivered securely<br/>to wallet</div>
              </div>
            </div>
          </div>
        </section>

        {/* Section 4: Threat Model table */}
        <section className="overflow-x-auto rounded-2xl border border-gray-800 bg-gray-900/30">
          <table className="w-full text-left text-sm whitespace-nowrap">
            <thead>
              <tr className="border-b border-gray-800 bg-gray-950/80">
                <th className="px-6 py-4 font-space text-gray-300 font-bold uppercase tracking-wider">Threat</th>
                <th className="px-6 py-4 font-space text-red-400 font-bold uppercase tracking-wider">Without Z-RWA</th>
                <th className="px-6 py-4 font-space text-green-400 font-bold uppercase tracking-wider">With Z-RWA</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-800/50">
              <tr className="hover:bg-gray-800/20 transition-colors">
                <td className="px-6 py-4 font-mono text-gray-300">Aadhaar/PAN leaked on-chain</td>
                <td className="px-6 py-4 text-gray-400 bg-red-950/10">Yes &mdash; stored directly</td>
                <td className="px-6 py-4 text-green-400 font-medium bg-green-950/10">Never &mdash; ZK proof only</td>
              </tr>
              <tr className="hover:bg-gray-800/20 transition-colors bg-black/20">
                <td className="px-6 py-4 font-mono text-gray-300">Payment amount visible</td>
                <td className="px-6 py-4 text-gray-400 bg-red-950/10">Yes &mdash; public ledger</td>
                <td className="px-6 py-4 text-green-400 font-medium bg-green-950/10">No &mdash; MagicBlock shielded</td>
              </tr>
              <tr className="hover:bg-gray-800/20 transition-colors">
                <td className="px-6 py-4 font-mono text-gray-300">KYC provider can sell data</td>
                <td className="px-6 py-4 text-gray-400 bg-red-950/10">Yes &mdash; centralized</td>
                <td className="px-6 py-4 text-green-400 font-medium bg-green-950/10">No &mdash; no data transmitted</td>
              </tr>
              <tr className="hover:bg-gray-800/20 transition-colors bg-black/20">
                <td className="px-6 py-4 font-mono text-gray-300">Regulatory verification</td>
                <td className="px-6 py-4 text-gray-400 bg-red-950/10">Manual, slow</td>
                <td className="px-6 py-4 text-green-400 font-medium bg-green-950/10">Instant ZK proof check</td>
              </tr>
            </tbody>
          </table>
        </section>

        {/* Section 5: Links */}
        <section className="flex flex-col sm:flex-row items-center justify-center gap-6 pb-20">
          <a href="https://github.com/DSHIVAAY-23/Z-RWA-Monorepo" target="_blank" rel="noreferrer" className="px-8 py-3 rounded-xl border border-gray-700 bg-gray-800 hover:bg-gray-700 hover:border-gray-600 transition-all text-white font-space font-semibold shadow-lg min-w-[200px] text-center">
            View on GitHub
          </a>
          <a href="https://z-rwa-monorepo-fzeb4r6c1-dshivaay23s-projects.vercel.app/" target="_blank" rel="noreferrer" className="px-8 py-3 rounded-xl border border-purple-500/50 bg-purple-600 hover:bg-purple-500 transition-all text-white font-space font-semibold shadow-[0_0_20px_rgba(168,85,247,0.3)] min-w-[200px] text-center">
            Live Demo
          </a>
          <Link href="/PRIVACY.md" className="px-8 py-3 rounded-xl border border-gray-700 bg-transparent hover:bg-gray-800 transition-all text-gray-300 font-space font-semibold min-w-[200px] text-center">
            Read PRIVACY.md
          </Link>
        </section>

      </main>
    </div>
  );
}
