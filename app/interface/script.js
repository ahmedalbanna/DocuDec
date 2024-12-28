document.getElementById('projectForm').addEventListener('submit', async (event) => {
    event.preventDefault();
  
    const formData = {
      projectTitle: document.getElementById('projectTitle').value,
      studentName: document.getElementById('studentName').value,
      department: document.getElementById('department').value,
      university: document.getElementById('university').value,
      date: document.getElementById('date').value,
      email: document.getElementById('email').value,
      // ... add other form fields as needed ...
    };
  
    try {
      const response = await fetch('/generate_project/', { // Replace /generate_project/ with your FastAPI endpoint
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(formData)
      });
  
      if (!response.ok) {
        const errorData = await response.json();
        throw new Error(errorData.detail || 'Report generation failed.');
      }
  
      const data = await response.json();
      document.getElementById('result').innerHTML = `<p>Report generated successfully! Download: <a href="/download_report/${data.filename}">${data.filename}</a></p>`;
  
    } catch (error) {
      document.getElementById('result').innerHTML = `<p style="color:red;">Error: ${error.message}</p>`;
    }
  });
  

  