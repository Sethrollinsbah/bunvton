export async function GET({ request, platform }) {
	try {
		const r2 = platform.env.VTON;

		// Pagination (ensure valid page number)
		const url = new URL(request.url);
		const page = Math.max(1, parseInt(url.searchParams.get('page') ?? '1')); // Prevent page < 1
		const pageSize = 10; // Number of results per page
		const vtonId = url.searchParams.get('id'); // Folder identifier

		const result = await r2.list();
		// const result = await r2.list(listOptions);

		// `result.delimitedPrefixes` will contain the folders in the specified directory
		const folders = result.delimitedPrefixes;

		// Fetch total file count for pagination (optional if you need to paginate through folders)
		const totalFilesList = await r2.list({ limit: 1000 }); // Adjust limit if needed
		const totalFiles = totalFilesList.objects.length;

		// Calculate total pages if paginating (optional)
		const totalPages = Math.ceil(totalFiles / pageSize);

		console.log(folders);
		return new Response(
			JSON.stringify({
				folders, // Return only folder names (not the files themselves)
				pagination: { page, pageSize, totalPages, totalFiles } // Pagination info (if needed)
			}),
			{ status: 200 }
		);
	} catch (error) {
		console.error('Error in GET handler:', error); // Log for debugging
		return new Response(JSON.stringify({ error: 'Internal Server Error' }), { status: 500 });
	}
}

export async function PUT({ env, request, url, platform }) {
	try {
		const r2 = platform.env.VTON;
		const vtonId = await url.searchParams.get('vtonId');

		if (!vtonId) {
			return new Response(JSON.stringify({ error: 'Invalid vton ID' }), { status: 400 });
		}

		const formData = await request.formData();

		// Array of files to check and upload
		const fileTypes = ['result', 'model', 'garment'];
		const uploadResponses = [];

		// Check for each file in the form data
		for (const fileType of fileTypes) {
			const file = formData.get(fileType); // Check for 'result', 'model', or 'garment'

			if (file) {
				// Generate the filename for R2, using `vtonId` as folder and the fileType as the file name
				const filename = `${vtonId}___${fileType}.jpg`;

				// Upload the file to R2 (using the appropriate R2 API for your setup)
				await r2.put(filename, file);

				// Store the upload result for response
				uploadResponses.push({ fileType, filename });
			} else {
				// Log missing files, but don't throw an error since they might come later
				console.log(`File '${fileType}' not uploaded in this request.`);
			}
		}

		// If no files were uploaded, return an error
		if (uploadResponses.length === 0) {
			return new Response(JSON.stringify({ error: 'No valid files uploaded' }), { status: 400 });
		}

		// Return success response with filenames of uploaded files
		return new Response(
			JSON.stringify({
				message: 'Files uploaded successfully',
				uploadedFiles: uploadResponses // List of uploaded files
			}),
			{ status: 201 }
		);
	} catch (error) {
		console.error('Error in PUT handler:', error);
		return new Response(JSON.stringify({ error: 'Internal Server Error' }), { status: 500 });
	}
}

// // Authentication (same as before)
// let sessionId = locals.lucia.readBearerToken(request.headers.get('Authorization') ?? '');
// if (!sessionId) {
//   sessionId = locals.lucia.readSessionCookie(request.headers.get('Cookie') ?? '');
//   if (!sessionId) {
//     return new Response(JSON.stringify({ message: 'Unauthorized' }), { status: 401 });
//   }
// }
// const { user } = await locals.lucia.validateSession(sessionId);
// const userId = user.id;

// Check for valid userId (simplified)
// if (!userId) {
//   return new Response(JSON.stringify({ error: 'Invalid user ID' }), { status: 400 });
// }

// Get the file data from the request
// // import { count, eq } from 'drizzle-orm';
// import * as schema from '$lib/server/schema';
// import { drizzle } from 'drizzle-orm/d1';
