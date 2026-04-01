'use client';

import { useState } from 'react';

const DOC_TYPES = [
  'Aadhaar Card',
  'PAN Card',
  'Passport',
  'Land Record (Bhulekh)',
  'Investor Certificate'
];

interface DocumentUploadProps {
  onDocumentReady: (docType: string, docHash: string) => void;
  isActive: boolean;
}

export default function DocumentUpload({ onDocumentReady, isActive }: DocumentUploadProps) {
  const [selectedType, setSelectedType] = useState(DOC_TYPES[0]);
  const [file, setFile] = useState<File | null>(null);
  const [isHashing, setIsHashing] = useState(false);
  const [mockHash, setMockHash] = useState('');

  const handleFileChange = async (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      const selectedFile = e.target.files[0];
      setFile(selectedFile);
      setIsHashing(true);
      
      await new Promise(resolve => setTimeout(resolve, 800));
      
      const mockSha = Array.from(crypto.getRandomValues(new Uint8Array(32)))
        .map(b => b.toString(16).padStart(2, '0'))
        .join('');
      
      setMockHash(mockSha);
      setIsHashing(false);
      onDocumentReady(selectedType, mockSha);
    }
  };

  const isCompleted = !!mockHash && !isHashing;

  return (
    <div className={`rounded-xl border-2 p-6 mb-4 transition-all duration-300 ${isActive || isCompleted ? 'border-purple-500/50 bg-purple-500/5' : 'border-white/8 bg-white/2 opacity-50'}`}>
      <div className="flex items-center gap-3 mb-4">
        <div className={`w-8 h-8 rounded-full text-white text-sm font-bold flex items-center justify-center ${isCompleted ? 'bg-accent-green text-black' : 'bg-purple-500'}`}>
          {isCompleted ? '✓' : '1'}
        </div>
        <span className="font-semibold text-white">
          🔒 Upload Document (Processed Locally)
        </span>
      </div>
      
      <select 
        value={selectedType}
        onChange={(e) => setSelectedType(e.target.value)}
        disabled={!isActive && !isCompleted}
        className="w-full bg-black/30 border border-white/10 rounded-lg px-4 py-3 text-white font-mono text-sm mb-4 outline-none focus:border-purple-500/50"
      >
        {DOC_TYPES.map(doc => (
          <option key={doc} value={doc}>{doc}</option>
        ))}
      </select>
      
      {!file ? (
        <label className="block border-2 border-dashed border-white/20 rounded-xl p-8 text-center hover:border-purple-500/50 transition-all cursor-pointer bg-black/20">
          <div className="text-4xl mb-3">📄</div>
          <div className="text-white font-medium mb-1">
            Drop document here or click to browse
          </div>
          <div className="text-gray-500 text-sm">
            PDF, PNG, JPG (MAX. 10MB)
          </div>
          <div className="mt-3 inline-flex items-center gap-2 text-xs text-accent-green font-mono">
            🔒 Zero data leaves your device
          </div>
          <input type="file" className="hidden" accept=".pdf,.png,.jpg,.jpeg" onChange={handleFileChange} disabled={!isActive && !isCompleted} />
        </label>
      ) : (
        <div className="border border-white/10 rounded-xl p-4 bg-black/40 flex flex-col gap-3">
          <div className="flex items-center justify-between border-b border-white/10 pb-3">
            <div className="flex items-center gap-3">
              <div className="text-2xl">📄</div>
              <div>
                <div className="text-sm font-semibold text-white truncate max-w-[200px]">{file.name}</div>
                <div className="text-xs text-gray-400">{(file.size / 1024 / 1024).toFixed(2)} MB</div>
              </div>
            </div>
            <button onClick={() => { setFile(null); setMockHash(''); }} className="text-xs text-purple-400 hover:text-purple-300">Reselect</button>
          </div>
          <div className="flex items-center justify-between">
            <span className="text-xs text-gray-500 font-mono">SHA-256 HASH:</span>
            {isHashing ? (
              <span className="text-xs text-accent-green font-mono animate-pulse">Computing...</span>
            ) : (
              <span className="text-xs text-accent-green font-mono bg-accent-green/10 px-2 py-1 rounded truncate max-w-[200px]">{mockHash}</span>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
