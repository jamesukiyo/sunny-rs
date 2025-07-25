class SunnyCli < Formula
	desc "Check the weather from your terminal."
	homepage "https://github.com/jamesukiyo/sunny-rs"
	license "MIT"

	on_macos do
		if Hardware::CPU.intel?
			url "https://github.com/jamesukiyo/sunny-rs/releases/download/v0.4.1/sunny-cli-darwin-x64.tar.gz"
			sha256 ""
		end
		if Hardware::CPU.arm?
			url "https://github.com/jamesukiyo/sunny-rs/releases/download/v0.4.1/sunny-cli-darwin-arm64.tar.gz"
			sha256 ""
		end
	end

	on_linux do
		if Hardware::CPU.intel?
			url "https://github.com/jamesukiyo/sunny-rs/releases/download/v0.4.1/sunny-cli-linux-x64.tar.gz"
			sha256 ""
		end
		if Hardware::CPU.arm?
			url "https://github.com/jamesukiyo/sunny-rs/releases/download/v0.4.1/sunny-cli-linux-arm64.tar.gz"
			sha256 ""
		end
	end

	def install
		bin.install "sunny"
	end

	test do
		system "#{bin}/sunny", "--version"
	end
end
