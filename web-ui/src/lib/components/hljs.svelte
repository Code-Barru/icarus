<script>
	import hljs from 'highlight.js';
	import 'highlight.js/styles/github.css';
	import { storeHighlightJs } from '@skeletonlabs/skeleton';

	import shell from 'highlight.js/lib/languages/shell';

	hljs.registerLanguage('shell', shell);
	storeHighlightJs.set(hljs);
</script>
