<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST User</name>
   <tag></tag>
   <elementGuidId>ddc4a98d-ecf0-4b47-a219-ad642321048a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;user-${$timestamp}\&quot;,\n    \&quot;email\&quot;: \&quot;${$timestamp}-user@gmail.com\&quot;,\n    \&quot;password\&quot;: \&quot;password\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${accessToken}</value>
      <webElementGuid>5b4cfffa-e873-4446-8b8a-ee4476625825</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name></name>
      <type>Main</type>
      <value></value>
      <webElementGuid>7c2e93e1-12ba-4737-96e6-10cf1640e033</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.9</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${host}/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.host</defaultValue>
      <description></description>
      <id>09a91d06-d5f0-4962-9275-437ad1e98088</id>
      <masked>false</masked>
      <name>host</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.accessToken</defaultValue>
      <description></description>
      <id>23181a51-f79c-4637-a866-73d9bc09b8dd</id>
      <masked>false</masked>
      <name>accessToken</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.$timestamp</defaultValue>
      <description></description>
      <id>938bdebc-6cf5-4f6e-bd91-fa2efe7f7226</id>
      <masked>false</masked>
      <name>$timestamp</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>