import { Shield, FileWarning, Fingerprint, MapPin, Award } from 'lucide-react';

export default function IndiaCompliance() {
  return (
    <div className="glass-panel p-8 mt-12 bg-gradient-to-br from-navy-800 to-navy-900 border-t-2 border-t-primary-500">
      <div className="flex items-center gap-3 mb-6">
        <span className="text-3xl">🇮🇳</span>
        <h2 className="text-2xl font-heading font-bold text-white">
          Built for India's RWA Revolution
        </h2>
      </div>

      <div className="grid md:grid-cols-2 gap-8">
        <div className="flex flex-col gap-3">
          <div className="flex items-center gap-2 text-orange-400">
            <FileWarning className="w-5 h-5" />
            <h3 className="font-semibold text-lg">The Problem:</h3>
          </div>
          <p className="text-slate-300 leading-relaxed text-sm">
            Indian farmers and landowners hold trillions in assets that
            cannot be tokenized without exposing highly sensitive Aadhaar or PAN details on-chain. Public ledgers and strict KYC requirements clash with privacy fundamental rights.
          </p>
        </div>

        <div className="flex flex-col gap-3">
          <div className="flex items-center gap-2 text-success-400">
            <Shield className="w-5 h-5" />
            <h3 className="font-semibold text-lg">Our Solution:</h3>
          </div>
          <ul className="text-slate-300 text-sm space-y-2">
            <li className="flex items-start gap-2">
              <span className="text-success-500 mt-0.5">✓</span>
              <span><strong>ZK proof</strong> certifies document validity completely locally.</span>
            </li>
            <li className="flex items-start gap-2">
              <span className="text-success-500 mt-0.5">✓</span>
              <span><strong>Document hash:</strong> Published on-chain for verifiability.</span>
            </li>
            <li className="flex items-start gap-2">
              <span className="text-success-500 mt-0.5">✓</span>
              <span><strong>Document content:</strong> NEVER leaves the user's device.</span>
            </li>
          </ul>
        </div>
      </div>

      <div className="mt-8 pt-6 border-t border-slate-700/50 text-sm">
        <h4 className="font-semibold text-slate-400 mb-4">Supported Document Types:</h4>
        <div className="flex flex-wrap gap-3">
          <div className="flex items-center gap-2 px-3 py-1.5 rounded bg-navy-800 border border-slate-700 text-slate-300">
            <Fingerprint className="w-4 h-4 text-primary-400" /> Aadhaar
          </div>
          <div className="flex items-center gap-2 px-3 py-1.5 rounded bg-navy-800 border border-slate-700 text-slate-300">
            <span className="font-bold text-primary-400 font-mono text-xs">PAN</span> PAN Card
          </div>
          <div className="flex items-center gap-2 px-3 py-1.5 rounded bg-navy-800 border border-slate-700 text-slate-300">
            <MapPin className="w-4 h-4 text-primary-400" /> Land Records
          </div>
          <div className="flex items-center gap-2 px-3 py-1.5 rounded bg-navy-800 border border-slate-700 text-slate-300">
            <Award className="w-4 h-4 text-primary-400" /> Investor Certificate
          </div>
        </div>
      </div>
    </div>
  );
}
